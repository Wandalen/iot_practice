use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, PartialEq, PartialOrd, Type, Deserialize, Serialize)]
#[sqlx(type_name = "staff_role", rename_all = "lowercase")]
pub enum StaffRole {
    Manager,
    Staff,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StaffAccount {
    pub id: i32,
    pub login: String,
    pub password_hash: String,
    pub role: StaffRole,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateStaffAccount {
    pub login: String,
    pub password_hash: String,
    pub role: StaffRole,
}
