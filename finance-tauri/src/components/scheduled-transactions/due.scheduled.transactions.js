import moment from "moment";

const dueScheduledTransactions = (allScheduledTransactions) => {
    let dueCount = 0;

    for (let t of allScheduledTransactions) {
        let today = moment();
        let nextDate = moment(t.next_date);

        if (nextDate.isSameOrBefore(today)) {
            dueCount++;
        }
    }

    return dueCount;
};

export {
    dueScheduledTransactions
};