use anyhow::Result;
use sqlx::SqlitePool;
use flowmason_auth::User;

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.created_at.to_rfc3339(),
            user.updated_at.to_rfc3339()
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query!(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = ?1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.id,
                email: row.email,
                password_hash: row.password_hash,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<User>> {
        let row = sqlx::query!(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.id,
                email: row.email,
                password_hash: row.password_hash,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }
}

