import {authenticationService} from "./authentication.service";

const getAllCategories = async () => {
    try {
        return await authenticationService.getWithAuth('/categories');
    } catch (e) {
        console.log(e);
        throw(e);
    }
};

const getAllCategoriesByType = async (categoryType) => {
    try {
        return await authenticationService.getWithAuth(`/categories/${categoryType}`);
    } catch (e) {
        console.log(e);
        throw(e);
    }
};

const getCategoryById = async (categoryId) => {
    try {
        return await authenticationService.getWithAuth(`/categories/${categoryId}`);
    } catch (e) {
        throw(e);
    }
};

const newCategory = async (name, categorytype) => {
    const category = await authenticationService.postWithAuth('/categories', {
        name,
        categorytype
    });

    return category.data;
};

const editCategoryById = async (categoryId, name, categorytype) => {
    const category = await authenticationService.patchWithAuth(`/categories/${categoryId}`, {
        name,
        categorytype
    });

    return category.data;
};

const deleteCategoryById = async (categoryId) => {
    try {
        return await authenticationService.deleteWithAuth(`/categories/${categoryId}`);
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
