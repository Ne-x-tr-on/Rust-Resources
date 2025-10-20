use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Books
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub published_at: Option<chrono::NaiveDate>,
}

// Members
#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

// Loans
#[derive(Debug, Serialize, Deserialize)]
pub struct Loan {
    pub id: Uuid,
    pub book_id: Uuid,
    pub member_id: Uuid,
    pub loaned_at: chrono::NaiveDateTime,
    pub returned_at: Option<chrono::NaiveDateTime>,
}
