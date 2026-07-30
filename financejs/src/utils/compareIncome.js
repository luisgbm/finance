import {parseTxDate, projectOccurrences} from './cashflow';

// Aggregate income transactions into a single total per category (integer cents) for the
// "Compare Income Categories" report. Only 'Income' transactions are counted; the category
// filter matches selectedCategoryIds (null = all income categories). Categories with no
// income in the range are omitted so the chart only shows meaningful slices/bars. Returns
// { labels, values } sorted by descending total (index-aligned).
export function computeIncomesByCategory(transactions, range, selectedCategoryIds) {
    const selectedSet = selectedCategoryIds === null ? null : new Set(selectedCategoryIds);
    const totals = new Map();

    for (const t of transactions) {
        if (t.category_type !== 'Income') continue;
        if (selectedSet && !selectedSet.has(t.category_id)) continue;
        const d = parseTxDate(t.date);
        if (range.start && d.isBefore(range.start)) continue;
        if (range.end && d.isAfter(range.end)) continue;

        const existing = totals.get(t.category_id);
        if (existing) {
            existing.value += t.value;
        } else {
            totals.set(t.category_id, {name: t.category_name || 'Uncategorized', value: t.value});
        }
    }

    const entries = [...totals.values()]
        .filter(e => e.value > 0)
        .sort((a, b) => b.value - a.value);

    return {
        labels: entries.map(e => e.name),
        values: entries.map(e => e.value),
    };
}

// Forecast variant: project scheduled income transactions into the range, then aggregate
// them by category exactly like the historical path. Only scheduled *Transactions* of type
// Income are counted (scheduled transfers are internal and excluded); the account is
// irrelevant here since this report groups by category, not account.
export function computeForecastIncomesByCategory(scheduledTransactions, range, selectedCategoryIds) {
    const selectedSet = selectedCategoryIds === null ? null : new Set(selectedCategoryIds);
    const synthetic = [];

    for (const st of scheduledTransactions) {
        if (st.kind !== 'Transaction') continue;
        if (st.category_type !== 'Income') continue;
        if (selectedSet && !selectedSet.has(st.category_id)) continue;

        const occurrences = projectOccurrences(st, range.start, range.end);
        for (const d of occurrences) {
            synthetic.push({
                date: d.format('YYYY-MM-DD HH:mm:ss'),
                category_type: 'Income',
                category_id: st.category_id,
                category_name: st.category_name,
                value: st.value,
            });
        }
    }

    return computeIncomesByCategory(synthetic, range, selectedCategoryIds);
}
