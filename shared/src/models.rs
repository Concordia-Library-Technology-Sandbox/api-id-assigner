use serde::{Deserialize,Serialize};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Node {
    pub id: i32,
    pub ip: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Default)]
pub struct DeleteNode {
    pub ip: String,
}

