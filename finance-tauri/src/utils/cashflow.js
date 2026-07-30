import moment from 'moment';

// Option lists shared by the Cashflow report controls.
export const BREAKDOWN_OPTIONS = [
    {value: 'Days', label: 'Days'},
    {value: 'Weeks', label: 'Weeks'},
    {value: 'Months', label: 'Months'},
    {value: 'Quarters', label: 'Quarters'},
    {value: 'Years', label: 'Years'},
];

export const PERIOD_OPTIONS = [
    {value: 'this_month', label: 'This month'},
    {value: 'last_month', label: 'Last month'},
    {value: 'last_3_months', label: 'Last 3 months'},
    {value: 'last_6_months', label: 'Last 6 months'},
    {value: 'last_12_months', label: 'Last 12 months'},
    {value: 'this_year', label: 'This year'},
    {value: 'last_year', label: 'Last year'},
    {value: 'all_time', label: 'All time'},
    {value: 'custom', label: 'Custom period'},
];

// Future-facing periods for the Forecast tab.
export const FORECAST_PERIOD_OPTIONS = [
    {value: 'this_month', label: 'This month'},
    {value: 'next_month', label: 'Next month'},
    {value: 'next_3_months', label: 'Next 3 months'},
    {value: 'next_6_months', label: 'Next 6 months'},
    {value: 'next_12_months', label: 'Next 12 months'},
    {value: 'this_year', label: 'This year'},
    {value: 'custom', label: 'Custom period'},
];

// Transactions are stored as 'YYYY-MM-DD HH:MM:SS' in the Tauri SQLite layer; the original
// web API returns ISO 'YYYY-MM-DDTHH:mm:ss'. Accept both so this module is portable.
const TX_DATE_FORMATS = ['YYYY-MM-DD HH:mm:ss', moment.ISO_8601];
const DAY_FORMAT = 'YYYY-MM-DD';

export function parseTxDate(s) {
    return moment(s, TX_DATE_FORMATS);
}

// Map a period option to a concrete {start, end} range (moment objects). `start === null`
// means unbounded (All time) — the caller derives the real start from the data.
export function periodToRange(period, customStart, customEnd, now = moment()) {
    const end = now.clone().endOf('day');

    switch (period) {
        case 'this_month':
            return {start: now.clone().startOf('month'), end};
        case 'last_month': {
            const m = now.clone().subtract(1, 'month');
            return {start: m.clone().startOf('month'), end: m.clone().endOf('month')};
        }
        case 'last_3_months':
            // "Last N months" is the current month plus the N-1 before it, aligned to the
            // 1st of the month so a Months breakdown yields exactly N buckets (not N+1 with
            // an empty partial-month at the start).
            return {start: now.clone().subtract(2, 'months').startOf('month'), end};
        case 'last_6_months':
            return {start: now.clone().subtract(5, 'months').startOf('month'), end};
        case 'last_12_months':
            return {start: now.clone().subtract(11, 'months').startOf('month'), end};
        case 'this_year':
            return {start: now.clone().startOf('year'), end};
        case 'last_year': {
            const y = now.clone().subtract(1, 'year');
            return {start: y.clone().startOf('year'), end: y.clone().endOf('year')};
        }
        case 'all_time':
            return {start: null, end};
        case 'custom':
            return {
                start: customStart ? moment(customStart, DAY_FORMAT).startOf('day') : null,
                end: customEnd ? moment(customEnd, DAY_FORMAT).endOf('day') : end,
            };
        default:
            return {start: null, end};
    }
}

// Map a Forecast period option to a concrete {start, end} range (moment objects). Forecast
// ranges always look forward. "Next N months" is the current month plus the N-1 after it,
// aligned to month boundaries so a Months breakdown yields exactly N buckets.
export function forecastPeriodToRange(period, customStart, customEnd, now = moment()) {
    switch (period) {
        case 'this_month':
            return {start: now.clone().startOf('month'), end: now.clone().endOf('month')};
        case 'next_month': {
            const m = now.clone().add(1, 'month');
            return {start: m.clone().startOf('month'), end: m.clone().endOf('month')};
        }
        case 'next_3_months':
            return {start: now.clone().startOf('month'), end: now.clone().add(2, 'months').endOf('month')};
        case 'next_6_months':
            return {start: now.clone().startOf('month'), end: now.clone().add(5, 'months').endOf('month')};
        case 'next_12_months':
            return {start: now.clone().startOf('month'), end: now.clone().add(11, 'months').endOf('month')};
        case 'this_year':
            return {start: now.clone().startOf('year'), end: now.clone().endOf('year')};
        case 'custom':
            return {
                start: customStart ? moment(customStart, DAY_FORMAT).startOf('day') : null,
                end: customEnd ? moment(customEnd, DAY_FORMAT).endOf('day') : null,
            };
        default:
            return {start: now.clone().startOf('month'), end: now.clone().endOf('month')};
    }
}
const STEP_UNIT = {Days: 'day', Weeks: 'week', Months: 'month', Quarters: 'quarter', Years: 'year'};
// moment unit used to snap a date to the start of its bucket.
const START_UNIT = {Days: 'day', Weeks: 'isoWeek', Months: 'month', Quarters: 'quarter', Years: 'year'};

// The bucket a given date falls into: a stable `key` (for grouping) and a human `label`.
function bucketOf(d, breakdown) {
    switch (breakdown) {
        case 'Days':
            return {key: d.format('YYYY-MM-DD'), label: d.format('DD/MM/YYYY')};
        case 'Weeks': {
            const s = d.clone().startOf('isoWeek');
            return {key: s.format('GGGG-[W]WW'), label: s.format('DD/MM/YYYY')};
        }
        case 'Months':
            return {key: d.format('YYYY-MM'), label: d.format('MMM YYYY')};
        case 'Quarters':
            return {key: `${d.year()}-Q${d.quarter()}`, label: `Q${d.quarter()} ${d.year()}`};
        case 'Years':
            return {key: d.format('YYYY'), label: d.format('YYYY')};
        default:
            return {key: d.format('YYYY-MM'), label: d.format('MMM YYYY')};
    }
}

// Ordered list of every bucket between start and end (inclusive) so the chart shows a
// continuous axis with zero-valued gaps.
function generateBuckets(start, end, breakdown) {
    const startUnit = START_UNIT[breakdown] || 'month';
    const stepUnit = STEP_UNIT[breakdown] || 'month';
    const buckets = [];
    let cur = start.clone().startOf(startUnit);
    let guard = 0;

    while (cur.isSameOrBefore(end) && guard < 5000) {
        buckets.push(bucketOf(cur, breakdown));
        cur = cur.add(1, stepUnit);
        guard++;
    }

    return buckets;
}

// Aggregate income/expense transactions into per-bucket sums (in integer cents).
//
// `transactions` are TransactionTransferJoined rows; only 'Income'/'Expense' are counted
// (internal transfers, which are 'TransferIncome'/'TransferExpense', are excluded from
// cashflow). Returns { labels, incomes, expenses } with the three arrays index-aligned.
export function computeCashflow(transactions, breakdown, range) {
    const inRange = transactions.filter((t) => {
        if (t.category_type !== 'Income' && t.category_type !== 'Expense') return false;
        const d = parseTxDate(t.date);
        if (range.start && d.isBefore(range.start)) return false;
        if (range.end && d.isAfter(range.end)) return false;
        return true;
    });

    let start = range.start;
    let end = range.end || moment();

    // All time: derive the start from the earliest transaction actually present.
    if (!start) {
        if (inRange.length === 0) return {labels: [], incomes: [], expenses: []};
        start = inRange.reduce((min, t) => {
            const d = parseTxDate(t.date);
            return d.isBefore(min) ? d : min;
        }, parseTxDate(inRange[0].date));
    }

    const buckets = generateBuckets(start, end, breakdown);
    const indexByKey = new Map(buckets.map((b, i) => [b.key, i]));
    const incomes = new Array(buckets.length).fill(0);
    const expenses = new Array(buckets.length).fill(0);

    for (const t of inRange) {
        const {key} = bucketOf(parseTxDate(t.date), breakdown);
        const i = indexByKey.get(key);
        if (i === undefined) continue;
        if (t.category_type === 'Income') incomes[i] += t.value;
        else expenses[i] += t.value;
    }

    return {labels: buckets.map((b) => b.label), incomes, expenses};
}

// moment duration unit for each repeat frequency (PascalCase over IPC: Days/Weeks/Months/Years).
const REPEAT_UNIT = {Days: 'days', Weeks: 'weeks', Months: 'months', Years: 'years'};

// Expand one scheduled transaction into the individual future occurrence dates that fall
// within [rangeStart, rangeEnd]. Honours non-repeating, finite-repeat (bounded by
// end_after_repeats minus current_repeat_count) and infinite-repeat schedules.
export function projectOccurrences(st, rangeStart, rangeEnd) {
    const occurrences = [];
    const first = parseTxDate(st.next_date);
    if (!first.isValid()) return occurrences;

    const within = (d) => (!rangeStart || !d.isBefore(rangeStart)) && (!rangeEnd || !d.isAfter(rangeEnd));

    if (!st.repeat) {
        if (within(first)) occurrences.push(first);
        return occurrences;
    }

    const unit = REPEAT_UNIT[st.repeat_freq] || 'months';
    const interval = st.repeat_interval && st.repeat_interval > 0 ? st.repeat_interval : 1;
    const remaining = st.infinite_repeat
        ? Infinity
        : Math.max(0, (st.end_after_repeats || 0) - (st.current_repeat_count || 0));

    let cur = first.clone();
    let made = 0;
    let guard = 0;
    while (made < remaining && guard < 100000) {
        guard++;
        if (rangeEnd && cur.isAfter(rangeEnd)) break;
        if (within(cur)) occurrences.push(cur.clone());
        cur = cur.clone().add(interval, unit);
        made++;
    }
    return occurrences;
}

// Aggregate projected scheduled transactions into per-bucket income/expense sums (integer
// cents), mirroring computeCashflow's output. Only scheduled *Transactions* of type
// Income/Expense are counted (scheduled transfers are internal and excluded); the account
// filter matches selectedAccountIds (null = all accounts).
export function computeForecast(scheduledTransactions, breakdown, range, selectedAccountIds) {
    const selectedSet = selectedAccountIds === null ? null : new Set(selectedAccountIds);
    const synthetic = [];

    for (const st of scheduledTransactions) {
        if (st.kind !== 'Transaction') continue;
        if (st.category_type !== 'Income' && st.category_type !== 'Expense') continue;
        if (selectedSet && !selectedSet.has(st.account_id)) continue;

        const occurrences = projectOccurrences(st, range.start, range.end);
        for (const d of occurrences) {
            synthetic.push({
                date: d.format('YYYY-MM-DD HH:mm:ss'),
                category_type: st.category_type,
                value: st.value,
            });
        }
    }

    return computeCashflow(synthetic, breakdown, range);
}
