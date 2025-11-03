async fn update_role(new_role:&str)-> Result<(),Box<dyn Error>>{
  println!("Updateting Role starting");

  if let Err(e) = fs::copy("xy.json","abc.json").await {
    println("Error doing my work mahn work on me");
  } else {
    println!("Backup successfully ccreated");
  }
}