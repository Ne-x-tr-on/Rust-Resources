-- Create Table
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100),
    author VARCHAR(50),
    published_at DATE
);

-- INSERT Data
INSERT INTO books (title, author, published_at)
VALUES ('Rust for Beginners', 'Newton', '2025-10-20');

-- inserting Multiple rows
INSERT INTO books (title, author, published_at)
VALUES 
('Learning SQL', 'Newton', '2025-10-21'),
('Mastering Rust', 'Afra', '2025-10-22');

-- use sqlx::PgPool;
-- use uuid::Uuid;

-- pub async fn create_book(pool: &PgPool, title: &str, author: &str) -> Result<(), sqlx::Error> {
--     let id = Uuid::new_v4(); // generate UUID for id
--     sqlx::query!(
--         "INSERT INTO books (id, title, author) VALUES ($1, $2, $3)",
--         id,
--         title,
--         author
--     )
--     .execute(pool)
--     .await?;
    
--     Ok(())
-- }

-- READ Data - Query data
SELECT * FROM books;
-- use sqlx::FromRow;

-- #[derive(Debug, FromRow)]
-- struct Book {
--     id: Uuid,
--     title: String,
--     author: String,
-- }

-- pub async fn get_all_books(pool: &PgPool) -> Result<Vec<Book>, sqlx::Error> {
--     let books = sqlx::query_as!(Book, "SELECT id, title, author FROM books")
--         .fetch_all(pool)
--         .await?;
    
--     Ok(books)
-- }

SELECT title, author FROM books;
-- pub async fn get_book_by_id(pool: &PgPool, id: Uuid) -> Result<Book, sqlx::Error> {
--     let book = sqlx::query_as!(Book, "SELECT id, title, author FROM books WHERE id = $1", id)
--         .fetch_one(pool)
--         .await?;
    
--     Ok(book)
-- }

SELECT * FROM books WHERE author = 'Newton';
SELECT * FROM books ORDER BY published_at DESC;



-- Update -Modify data
UPDATE books
SET title = 'Advanced Rust'
WHERE id = 1;

-- pub async fn update_book_title(pool: &PgPool, id: Uuid, new_title: &str) -> Result<(), sqlx::Error> {
--     sqlx::query!(
--         "UPDATE books SET title = $1 WHERE id = $2",
--         new_title,
--         id
--     )
--     .execute(pool)
--     .await?;
    
--     Ok(())
-- }


-- DELETE -Remove data
DELETE FROM books WHERE id = 1;
-- pub async fn delete_book(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
--     sqlx::query!("DELETE FROM books WHERE id = $1", id)
--         .execute(pool)
--         .await?;
    
--     Ok(())
-- }
