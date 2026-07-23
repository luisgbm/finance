use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::{Category, CategoryTypes};

const COLUMNS: &str = "id, categorytype, name, user_id";

pub async fn insert(
    pool: &SqlitePool,
    categorytype: CategoryTypes,
    name: &str,
    user_id: i32,
) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "INSERT INTO categories (categorytype, name, user_id) VALUES (?, ?, ?) RETURNING {COLUMNS}"
    ))
    .bind(categorytype)
    .bind(name)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn get_all(pool: &SqlitePool, user_id: i32) -> Result<Vec<Category>, AppError> {
    let categories = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE user_id = ? ORDER BY id"
    ))
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get_all_by_type(
    pool: &SqlitePool,
    category_type: CategoryTypes,
    user_id: i32,
) -> Result<Vec<Category>, AppError> {
    let categories = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE user_id = ? AND categorytype = ? ORDER BY id"
    ))
    .bind(user_id)
    .bind(category_type)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get(pool: &SqlitePool, id: i32, user_id: i32) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE user_id = ? AND id = ?"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn update(
    pool: &SqlitePool,
    id: i32,
    categorytype: CategoryTypes,
    name: &str,
    user_id: i32,
) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "UPDATE categories SET name = ?, categorytype = ? WHERE user_id = ? AND id = ? RETURNING {COLUMNS}"
    ))
    .bind(name)
    .bind(categorytype)
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn delete(pool: &SqlitePool, id: i32, user_id: i32) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "DELETE FROM categories WHERE user_id = ? AND id = ? RETURNING {COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}
