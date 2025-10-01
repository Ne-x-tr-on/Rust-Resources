// cargo add sqlx --features runtime-tokio,postgres,macros
use sqlx::postgres::PgPoolOptions;
// cargo add Error
use std::error::Error;
// cargo add tokio --features full
#[tokio::main]
async fn main(){
   if let Err(e) = read_db().await{
   println!("Error hile Reading from Database\n{:?}",e);
   }

   async fn read_db() -> Result<(),Box<dyn Error>>{
    // let url = dburl
    let url = "postgres://myuser:mypass@localhost:5432/mydb";
    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    // insert people in my db
    // sqlx::query("INSERT INTO users (name,email) VALUES($1,$2)")
    //     .bind("David")
    //     .bind("david@kamau.com")
    //     .execute(&_pool)
    //     .await?;

    Ok(())
   }
}
