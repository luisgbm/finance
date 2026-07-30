import moment from 'moment';
import {parseTxDate, projectOccurrences} from './cashflow';

// The Statistics report summarises a set of accounts over a period into a handful of numeric
// cards (rather than a chart). Everything here works in integer cents.
//
// A transaction/transfer leg's `value` is always positive; the category type carries the
// direction. `balanceEffect` returns its signed effect on the balance of its own account:
// Income/TransferIncome add, Expense/TransferExpense subtract. Because a transfer between two
// selected accounts contributes both legs (a +value TransferIncome and a -value TransferExpense),
// summing effects over every fetched leg correctly nets internal transfers to zero while still
// counting transfers that cross the selected-account boundary.
function balanceEffect(categoryType, value) {
    return (categoryType === 'Income' || categoryType === 'TransferIncome') ? value : -value;
}

// Express a period total as per-day / per-month / per-year *rates* over the selected period.
// Months and years use average calendar lengths (365.25 / 12 and 365.25 days) so arbitrary
// custom ranges work, not just whole months.
function averages(total, days) {
    const d = Math.max(1, days);
    return {
        total,
        daily: Math.round(total / d),
        monthly: Math.round(total / (d / (365.25 / 12))),
        yearly: Math.round(total / (d / 365.25)),
    };
}

// Top `n` categories (by summed value, descending) for a given category type. Each entry is
// {name, value} in integer cents. `legs` carry {category_type, category_id, category_name, value}.
function topCategories(legs, type, n) {
    const totals = new Map();
    for (const t of legs) {
        if (t.category_type !== type) continue;
        const key = t.category_id == null ? `name:${t.category_name}` : t.category_id;
        const cur = totals.get(key) || {name: t.category_name || 'Uncategorized', value: 0};
        cur.value += t.value;
        totals.set(key, cur);
    }
    return [...totals.values()]
        .filter((e) => e.value > 0)
        .sort((a, b) => b.value - a.value)
        .slice(0, n);
}

// Count expenses/incomes per row and transfers deduped by transfer id (both legs of an internal
// transfer share the same id, so it is counted once).
function transactionCounts(legs) {
    let expenses = 0;
    let incomes = 0;
    const transferIds = new Set();
    for (const t of legs) {
        if (t.category_type === 'Expense') expenses++;
        else if (t.category_type === 'Income') incomes++;
        else transferIds.add(t.id);
    }
    return {expenses, incomes, transfers: transferIds.size};
}

// Reconstruct the net-worth timeline from the full transaction history of the selected accounts
// (anchored to the accounts' current balances) and return the highest and lowest net worth that
// held at any point inside [rangeStart, rangeEnd]. A null `rangeStart` means "from the beginning
// of history". The history is walked forwards from the net worth that held before the first
// transaction (currentNetWorth minus the sum of every effect).
function netWorthExtremes(allLegs, currentNetWorth, rangeStart, rangeEnd) {
    const legs = [...allLegs].sort(
        (a, b) => parseTxDate(a.date).valueOf() - parseTxDate(b.date).valueOf()
    );
    const totalEffect = legs.reduce((s, t) => s + balanceEffect(t.category_type, t.value), 0);

    let running = currentNetWorth - totalEffect;
    let atStart = running;
    const startMs = rangeStart ? rangeStart.valueOf() : -Infinity;
    const endMs = rangeEnd ? rangeEnd.valueOf() : Infinity;

    let hi = -Infinity;
    let lo = Infinity;
    for (const leg of legs) {
        running += balanceEffect(leg.category_type, leg.value);
        const ms = parseTxDate(leg.date).valueOf();
        if (ms <= startMs) {
            atStart = running;
        } else if (ms <= endMs) {
            if (running > hi) hi = running;
            if (running < lo) lo = running;
        }
    }

    // Fold in the value that held at the very start of the window so periods with no
    // transactions still report the flat net worth.
    hi = Math.max(hi, atStart);
    lo = Math.min(lo, atStart);
    return {highest: hi, lowest: lo};
}

// Build the Historical statistics card data. `transactions` is the *full* history of the selected
// accounts (TransactionTransferJoined rows), `range` is {start, end} moments (start may be null
// for All time), `currentNetWorth` is the sum of the selected accounts' current balances.
export function computeHistoricalStatistics(transactions, range, currentNetWorth) {
    const end = range.end || moment();

    const inRange = transactions.filter((t) => {
        const d = parseTxDate(t.date);
        if (range.start && d.isBefore(range.start)) return false;
        if (d.isAfter(end)) return false;
        return true;
    });

    // All time (start === null): the effective start is the earliest transaction present.
    let start = range.start;
    if (!start) {
        start = inRange.reduce((min, t) => {
            const d = parseTxDate(t.date);
            return min === null || d.isBefore(min) ? d : min;
        }, null) || end.clone();
    }

    const expenseTotal = inRange
        .filter((t) => t.category_type === 'Expense')
        .reduce((s, t) => s + t.value, 0);
    const incomeTotal = inRange
        .filter((t) => t.category_type === 'Income')
        .reduce((s, t) => s + t.value, 0);

    const days = end.diff(start, 'days') + 1;
    const {highest, lowest} = netWorthExtremes(transactions, currentNetWorth, start, end);

    return {
        kind: 'statistics',
        expenses: averages(expenseTotal, days),
        incomes: averages(incomeTotal, days),
        topExpenseCategories: topCategories(inRange, 'Expense', 5),
        topIncomeCategories: topCategories(inRange, 'Income', 5),
        currentNetWorth,
        highestNetWorth: highest,
        lowestNetWorth: lowest,
        counts: transactionCounts(inRange),
        periodStart: start.format('YYYY-MM-DD'),
        periodEnd: end.format('YYYY-MM-DD'),
    };
}

// Build the Forecast statistics card data by projecting the scheduled transactions forward.
// `range` is {start, end} moments (from forecastPeriodToRange). Period stats come from occurrences
// inside the selected period; the net-worth trajectory is projected from today's currentNetWorth
// through the period end.
export function computeForecastStatistics(scheduledTransactions, range, selectedAccountIds, currentNetWorth) {
    const selectedSet = selectedAccountIds === null ? null : new Set(selectedAccountIds);
    const accountSelected = (id) => id != null && (selectedSet === null || selectedSet.has(id));

    const now = moment();
    const start = range.start || now.clone();
    const end = range.end || now.clone();

    const expenseLegs = [];   // {category_type, category_id, category_name, value}
    const incomeLegs = [];
    let transferCount = 0;
    const nwEvents = [];       // {ms, effect}

    for (const st of scheduledTransactions) {
        if (st.kind === 'Transaction') {
            if (st.category_type !== 'Income' && st.category_type !== 'Expense') continue;
            if (!accountSelected(st.account_id)) continue;

            for (const d of projectOccurrences(st, start, end)) {  // eslint-disable-line no-unused-vars
                const leg = {
                    category_type: st.category_type,
                    category_id: st.category_id,
                    category_name: st.category_name,
                    value: st.value,
                };
                if (st.category_type === 'Expense') expenseLegs.push(leg);
                else incomeLegs.push(leg);
            }

            const effect = balanceEffect(st.category_type, st.value);
            for (const d of projectOccurrences(st, now, end)) {
                nwEvents.push({ms: d.valueOf(), effect});
            }
        } else if (st.kind === 'Transfer') {
            const originSel = accountSelected(st.origin_account_id);
            const destSel = accountSelected(st.destination_account_id);
            if (!originSel && !destSel) continue;

            transferCount += projectOccurrences(st, start, end).length;

            // Effect on the *selected* net worth: +value if it lands in a selected account,
            // -value if it leaves one; an internal transfer (both selected) nets to zero.
            const effect = (destSel ? st.value : 0) + (originSel ? -st.value : 0);
            for (const d of projectOccurrences(st, now, end)) {
                nwEvents.push({ms: d.valueOf(), effect});
            }
        }
    }

    const expenseTotal = expenseLegs.reduce((s, t) => s + t.value, 0);
    const incomeTotal = incomeLegs.reduce((s, t) => s + t.value, 0);
    const days = end.diff(start, 'days') + 1;

    // Net-worth trajectory from today forward; highest/lowest across the projected horizon
    // (currentNetWorth itself is always a candidate — it is where the trajectory starts).
    nwEvents.sort((a, b) => a.ms - b.ms);
    let running = currentNetWorth;
    let hi = currentNetWorth;
    let lo = currentNetWorth;
    for (const ev of nwEvents) {
        running += ev.effect;
        if (running > hi) hi = running;
        if (running < lo) lo = running;
    }

    const allLegs = [...expenseLegs, ...incomeLegs];

    return {
        kind: 'statistics',
        expenses: averages(expenseTotal, days),
        incomes: averages(incomeTotal, days),
        topExpenseCategories: topCategories(allLegs, 'Expense', 5),
        topIncomeCategories: topCategories(allLegs, 'Income', 5),
        currentNetWorth,
        highestNetWorth: hi,
        lowestNetWorth: lo,
        counts: {expenses: expenseLegs.length, incomes: incomeLegs.length, transfers: transferCount},
        periodStart: start.format('YYYY-MM-DD'),
        periodEnd: end.format('YYYY-MM-DD'),
    };
}
