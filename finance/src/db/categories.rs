use sqlx::PgPool;

use crate::error::AppError;
use crate::models::{Category, CategoryTypes};

const COLUMNS: &str = "id, categorytype, name, user_id";

pub async fn insert(
    pool: &PgPool,
    categorytype: CategoryTypes,
    name: &str,
    user_id: i32,
) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "INSERT INTO categories (categorytype, name, user_id) VALUES ($1, $2, $3) RETURNING {COLUMNS}"
    ))
    .bind(categorytype)
    .bind(name)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn get_all(pool: &PgPool, user_id: i32) -> Result<Vec<Category>, AppError> {
    let categories = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE user_id = $1 ORDER BY id"
    ))
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get_all_by_type(
    pool: &PgPool,
    category_type: CategoryTypes,
    user_id: i32,
) -> Result<Vec<Category>, AppError> {
    let categories = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE user_id = $1 AND categorytype = $2 ORDER BY id"
    ))
    .bind(user_id)
    .bind(category_type)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get(pool: &PgPool, id: i32, user_id: i32) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "SELECT {COLUMNS} FROM categories WHERE user_id = $1 AND id = $2"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    categorytype: CategoryTypes,
    name: &str,
    user_id: i32,
) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "UPDATE categories SET name = $1, categorytype = $2 WHERE user_id = $3 AND id = $4 RETURNING {COLUMNS}"
    ))
    .bind(name)
    .bind(categorytype)
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<Category, AppError> {
    let category = sqlx::query_as::<_, Category>(&format!(
        "DELETE FROM categories WHERE user_id = $1 AND id = $2 RETURNING {COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}
