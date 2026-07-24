use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::{Category, CategoryTypes};

const COLUMNS: &str = "id, categorytype, name";

pub async fn insert(
    pool: &SqlitePool,
    categorytype: CategoryTypes,
    name: &str,
) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "INSERT INTO categories (categorytype, name) VALUES (?, ?) RETURNING {COLUMNS}"
    ))
    .bind(categorytype)
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Category>, AppError> {
    let categories = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories ORDER BY id"
    ))
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get_all_by_type(
    pool: &SqlitePool,
    category_type: CategoryTypes,
) -> Result<Vec<Category>, AppError> {
    let categories = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE categorytype = ? ORDER BY id"
    ))
    .bind(category_type)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get(pool: &SqlitePool, id: i32) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE id = ?"
    ))
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
) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "UPDATE categories SET name = ?, categorytype = ? WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(name)
    .bind(categorytype)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn delete(pool: &SqlitePool, id: i32) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "DELETE FROM categories WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}
