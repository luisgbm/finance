import { commands, call } from './finance';
import type { CategoryTypes } from './bindings';

const getAllCategories = async () => {
    try {
        return await call(commands.getCategories());
    } catch (e) {
        console.log(e);
        throw e;
    }
};

const getAllCategoriesByType = async (categoryType: string) => {
    try {
        return await call(commands.getCategoriesByType(categoryType));
    } catch (e) {
        console.log(e);
        throw e;
    }
};

const getCategoryById = async (categoryId: number) => {
    return await call(commands.getCategory(categoryId));
};

const newCategory = async (name: string, categorytype: CategoryTypes) => {
    const { data } = await call(commands.createCategory({ name, categorytype }));
    return data;
};

const editCategoryById = async (categoryId: number, name: string, categorytype: CategoryTypes) => {
    const { data } = await call(commands.updateCategory(categoryId, { name, categorytype }));
    return data;
};

const deleteCategoryById = async (categoryId: number) => {
    return await call(commands.deleteCategory(categoryId));
};

export const categoryService = {
    getAllCategories,
    getAllCategoriesByType,
    getCategoryById,
    newCategory,
    editCategoryById,
    deleteCategoryById,
};
