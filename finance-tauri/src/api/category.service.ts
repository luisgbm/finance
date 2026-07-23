import { commands, call, requireToken } from './finance';
import type { CategoryTypes } from './bindings';

const getAllCategories = async () => {
    try {
        return await call(commands.getCategories(requireToken()));
    } catch (e) {
        console.log(e);
        throw e;
    }
};

const getAllCategoriesByType = async (categoryType: string) => {
    try {
        return await call(commands.getCategoriesByType(requireToken(), categoryType));
    } catch (e) {
        console.log(e);
        throw e;
    }
};

const getCategoryById = async (categoryId: number) => {
    return await call(commands.getCategory(requireToken(), categoryId));
};

const newCategory = async (name: string, categorytype: CategoryTypes) => {
    const { data } = await call(commands.createCategory(requireToken(), { name, categorytype }));
    return data;
};

const editCategoryById = async (categoryId: number, name: string, categorytype: CategoryTypes) => {
    const { data } = await call(
        commands.updateCategory(requireToken(), categoryId, { name, categorytype }),
    );
    return data;
};

const deleteCategoryById = async (categoryId: number) => {
    return await call(commands.deleteCategory(requireToken(), categoryId));
};

export const categoryService = {
    getAllCategories,
    getAllCategoriesByType,
    getCategoryById,
    newCategory,
    editCategoryById,
    deleteCategoryById,
};
