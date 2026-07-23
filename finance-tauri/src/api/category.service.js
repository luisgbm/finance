import {authenticationService} from "./authentication.service";

const getAllCategories = async () => {
    try {
        return await authenticationService.callAuthed('get_categories');
    } catch (e) {
        console.log(e);
        throw(e);
    }
};

const getAllCategoriesByType = async (categoryType) => {
    try {
        return await authenticationService.callAuthed('get_categories_by_type', {categoryType});
    } catch (e) {
        console.log(e);
        throw(e);
    }
};

const getCategoryById = async (categoryId) => {
    try {
        return await authenticationService.callAuthed('get_category', {categoryId});
    } catch (e) {
        throw(e);
    }
};

const newCategory = async (name, categorytype) => {
    const category = await authenticationService.callAuthed('create_category', {
        req: {name, categorytype}
    });

    return category.data;
};

const editCategoryById = async (categoryId, name, categorytype) => {
    const category = await authenticationService.callAuthed('update_category', {
        categoryId,
        req: {name, categorytype}
    });

    return category.data;
};

const deleteCategoryById = async (categoryId) => {
    try {
        return await authenticationService.callAuthed('delete_category', {categoryId});
    } catch (e) {
        throw(e);
    }
};

export const categoryService = {
    getAllCategories,
    getAllCategoriesByType,
    getCategoryById,
    newCategory,
    editCategoryById,
    deleteCategoryById
};
