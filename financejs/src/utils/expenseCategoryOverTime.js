import moment from 'moment';
import {bucketOf, generateBuckets, parseTxDate, projectOccurrences} from './cashflow';

// Aggregate expense transactions in the selected categories into per-bucket sums (integer
// cents) for the "Expense Category Over Time" report. Only 'Expense' transactions whose
// category is in selectedCategoryIds (null = all expense categories) are counted; each bucket
// is a time slice defined by `breakdown` (Days/Weeks/Months/Quarters/Years). Returns
// { labels, values } with a continuous axis (empty buckets are zero), index-aligned.
export function computeExpenseCategoryOverTime(transactions, breakdown, range, selectedCategoryIds) {
    const selectedSet = selectedCategoryIds === null ? null : new Set(selectedCategoryIds);

    const inRange = transactions.filter((t) => {
        if (t.category_type !== 'Expense') return false;
        if (selectedSet && !selectedSet.has(t.category_id)) return false;
        const d = parseTxDate(t.date);
        if (range.start && d.isBefore(range.start)) return false;
        if (range.end && d.isAfter(range.end)) return false;
        return true;
    });

    let start = range.start;
    let end = range.end || moment();

    // All time: derive the start from the earliest matching transaction actually present.
    if (!start) {
        if (inRange.length === 0) return {labels: [], values: []};
        start = inRange.reduce((min, t) => {
            const d = parseTxDate(t.date);
            return d.isBefore(min) ? d : min;
        }, parseTxDate(inRange[0].date));
    }

    const buckets = generateBuckets(start, end, breakdown);
    const indexByKey = new Map(buckets.map((b, i) => [b.key, i]));
    const values = new Array(buckets.length).fill(0);

    for (const t of inRange) {
        const {key} = bucketOf(parseTxDate(t.date), breakdown);
        const i = indexByKey.get(key);
        if (i === undefined) continue;
        values[i] += t.value;
    }

    return {labels: buckets.map((b) => b.label), values};
}

// Forecast variant: project scheduled expense transactions in the selected categories into
// the range, then aggregate them per bucket exactly like the historical path. Only scheduled
// *Transactions* of type Expense are counted (scheduled transfers are internal and excluded).
export function computeForecastExpenseCategoryOverTime(scheduledTransactions, breakdown, range, selectedCategoryIds) {
    const selectedSet = selectedCategoryIds === null ? null : new Set(selectedCategoryIds);
    const synthetic = [];

    for (const st of scheduledTransactions) {
        if (st.kind !== 'Transaction') continue;
        if (st.category_type !== 'Expense') continue;
        if (selectedSet && !selectedSet.has(st.category_id)) continue;

        const occurrences = projectOccurrences(st, range.start, range.end);
        for (const d of occurrences) {
            synthetic.push({
                date: d.format('YYYY-MM-DD HH:mm:ss'),
                category_type: 'Expense',
                category_id: st.category_id,
                category_name: st.category_name,
                value: st.value,
            });
        }
    }

    return computeExpenseCategoryOverTime(synthetic, breakdown, range, selectedCategoryIds);
}
