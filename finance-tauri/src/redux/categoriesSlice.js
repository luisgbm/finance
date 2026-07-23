const initialState = [];

const categoriesReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'addCategory': {
            return [
                ...state,
                action.payload
            ].sort((a, b) => a.name.localeCompare(b.name));
        }
        case 'setCategories': {
            return action.payload.sort((a, b) => a.name.localeCompare(b.name));
        }
        case 'editCategory': {
            return state.map(category => {
                if (category.id === parseInt(action.payload.id)) {
                    return action.payload;
                }

                return category;
            }).sort((a, b) => a.name.localeCompare(b.name));
        }
        case 'deleteCategory': {
            return state.filter(category => category.id !== action.payload).sort((a, b) => a.name.localeCompare(b.name));
        }
        default:
            return state;
    }
};

export {
    categoriesReducer
}