use anyhow::Result;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use flowmason_auth::User;

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<()> {
        let created_at_str = user.created_at.to_rfc3339();
        let updated_at_str = user.updated_at.to_rfc3339();
        
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            user.id,
            user.email,
            user.password_hash,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = ?1
            "#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Self::row_to_user(&row)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = ?1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Self::row_to_user(&row)?))
        } else {
            Ok(None)
        }
    }

    pub async fn update(&self, user: &User) -> Result<()> {
        let updated_at_str = chrono::Utc::now().to_rfc3339();

        sqlx::query!(
            r#"
            UPDATE users
            SET email = ?1, password_hash = ?2, updated_at = ?3
            WHERE id = ?4
            "#,
            user.email,
            user.password_hash,
            updated_at_str,
            user.id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_all(&self) -> Result<Vec<User>> {
        let rows = sqlx::query(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(Self::row_to_user).collect()
    }

    fn row_to_user(row: &SqliteRow) -> Result<User> {
        let id: String = row.try_get("id")?;
        let email: String = row.try_get("email")?;
        let password_hash: String = row.try_get("password_hash")?;
        let created_at_str: String = row.try_get("created_at")?;
        let updated_at_str: String = row.try_get("updated_at")?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse created_at: {}", e))?
            .with_timezone(&chrono::Utc);
        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse updated_at: {}", e))?
            .with_timezone(&chrono::Utc);

        Ok(User {
            id,
            email,
            password_hash,
            created_at,
            updated_at,
        })
    }
}

