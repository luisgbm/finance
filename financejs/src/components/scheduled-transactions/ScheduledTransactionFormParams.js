import * as yup from "yup";
import moment from "moment";

const scheduledTransactionValidationSchema = yup.object({
    accountId: yup
        .number('Select the account')
        .required('Account is required'),
    value: yup
        .number('Enter the value')
        .moreThan(0, 'Value must be greater than 0')
        .required('Value is required'),
    categoryType: yup
        .string('Select the category type')
        .required('Type is required'),
    categoryId: yup
        .number('Select the category')
        .required('Category is required'),
    repeat: yup
        .boolean(),
    repeatFreq: yup
        .string()
        .when('repeat', {
            is: true,
            then: (schema) => schema.required('Frequency is required')
        }),
    repeatInterval: yup
        .number()
        .moreThan(0, 'Interval must be greater than 0')
        .when('repeat', {
            is: true,
            then: (schema) => schema.required('Interval is required')
        }),
    infiniteRepeat: yup
        .boolean(),
    endAfterRepeats: yup
        .number()
        .moreThan(0, 'End After Repetitions must be greater than 0')
        .when('repeat', {
            is: true,
            then: (schema) => schema.when('infiniteRepeat', {
                is: false,
                then: (s) => s.required('End After Repetitions is required')
            })
        })
});

const scheduledTransactionInitialValues = {
    value: '',
    description: '',
    accountId: '',
    categoryType: '',
    categoryId: '',
    createdDate: moment(),
    repeat: false,
    repeatFreq: '',
    repeatInterval: '',
    infiniteRepeat: false,
    endAfterRepeats: ''
};

export {
    scheduledTransactionValidationSchema,
    scheduledTransactionInitialValues
} ;