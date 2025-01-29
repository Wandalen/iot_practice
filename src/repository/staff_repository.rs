use sqlx::PgPool;

use crate::model::staff::{CreateStaffAccount, StaffAccount, StaffRole};

pub struct StaffRepository {
    pool: PgPool,
}

impl StaffRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, staff: &CreateStaffAccount) -> Result<i32, sqlx::Error> {
        let id: i32 = sqlx::query_scalar!(
            r#"
            INSERT INTO staff_accounts (login, password_hash, role)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            staff.login,
            staff.password_hash,
            staff.role.clone() as StaffRole
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn update(&self, staff: &StaffAccount) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE staff_accounts
            SET login = $1, password_hash = $2, role = $3
            WHERE id = $4
            "#,
            staff.login,
            staff.password_hash,
            staff.role.clone() as StaffRole,
            staff.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get(&self) -> Result<Vec<StaffAccount>, sqlx::Error> {
        let accounts = sqlx::query_as!(
            StaffAccount,
            r#"
            SELECT id, login, password_hash, role as "role: StaffRole"
            FROM staff_accounts
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(accounts)
    }
}
