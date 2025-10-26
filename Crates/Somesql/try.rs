CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100),
    author VARCHAR(50),
    published_at DATE
);

pub async fn create_book(pool:&PgPool,title:&str,author:&str)-> Result<(),sqlx::Error>{
  let id = Uuid::new_v4();
  sqlx::query!("
  INSERT INTO books(id,title,author) VALUES($1,$2,$3)",
  id,
  title,
  author
)
.execute(&pool)
.await?;

}

pub async fn get_book(pool&PgPool)->Result<(),sqlx::Error>