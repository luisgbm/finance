const moneyFormat = (value, supressCurrency) => {
    let valueInt = parseInt(value) / 100;

    if (supressCurrency) {
        let result = valueInt.toLocaleString("en-US", {style: "currency", currency: "USD"});
        return result.substring(1, result.length);
    } else {
        return valueInt.toLocaleString("en-US", {style: "currency", currency: "USD"});
    }
};

export {
    moneyFormat
};