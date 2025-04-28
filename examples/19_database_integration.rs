// Database Integration in Rust
// This example demonstrates how to work with databases using SQLx and PostgreSQL

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use serde::{Deserialize, Serialize};
use tokio;
use anyhow::Result;
use chrono::{DateTime, Utc};

// Define our data models with SQLx attributes for type-safe queries
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewUser {
    username: String,
    email: String,
}

// Database repository pattern
struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    // Create a new repository instance
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // Create a new user
    pub async fn create_user(&self, new_user: NewUser) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, created_at)
            VALUES ($1, $2, NOW())
            RETURNING id, username, email, created_at
            "#,
            new_user.username,
            new_user.email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    // Get a user by ID
    pub async fn get_user(&self, user_id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, created_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // List all users with pagination
    pub async fn list_users(&self, limit: i32, offset: i32) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, created_at
            FROM users
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    // Update a user
    pub async fn update_user(&self, user_id: i32, new_email: String) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1
            WHERE id = $2
            RETURNING id, username, email, created_at
            "#,
            new_email,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // Delete a user
    pub async fn delete_user(&self, user_id: i32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Example of a transaction
    pub async fn transfer_data(&self, from_id: i32, to_id: i32) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Perform multiple operations in a transaction
        sqlx::query!(
            r#"
            UPDATE users
            SET email = (SELECT email FROM users WHERE id = $1)
            WHERE id = $2
            "#,
            from_id,
            to_id
        )
        .execute(&mut tx)
        .await?;

        // If any operation fails, the transaction will be rolled back
        tx.commit().await?;
        Ok(())
    }
}

// Database migrations
async fn run_migrations(pool: &Pool<Postgres>) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    Ok(())
}

// Example usage
#[tokio::main]
async fn main() -> Result<()> {
    // Set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/dbname")
        .await?;

    // Run migrations
    run_migrations(&pool).await?;

    // Create repository
    let repo = UserRepository::new(pool);

    // Example operations
    let new_user = NewUser {
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
    };

    // Create a user
    let user = repo.create_user(new_user).await?;
    println!("Created user: {:?}", user);

    // Get user by ID
    if let Some(found_user) = repo.get_user(user.id).await? {
        println!("Found user: {:?}", found_user);
    }

    // List users
    let users = repo.list_users(10, 0).await?;
    println!("All users: {:?}", users);

    // Update user
    if let Some(updated_user) = repo.update_user(user.id, "newemail@example.com".to_string()).await? {
        println!("Updated user: {:?}", updated_user);
    }

    // Delete user
    let deleted = repo.delete_user(user.id).await?;
    println!("User deleted: {}", deleted);

    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_create_user() -> Result<()> {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://username:password@localhost/test_db")
            .await?;

        let repo = UserRepository::new(pool);
        let new_user = NewUser {
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
        };

        let user = repo.create_user(new_user).await?;
        assert_eq!(user.username, "test_user");
        assert_eq!(user.email, "test@example.com");

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_user() -> Result<()> {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://username:password@localhost/test_db")
            .await?;

        let repo = UserRepository::new(pool);
        let new_user = NewUser {
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
        };

        let created_user = repo.create_user(new_user).await?;
        let found_user = repo.get_user(created_user.id).await?.unwrap();
        
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.username, created_user.username);

        Ok(())
    }
} 