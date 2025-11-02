
async fn get_profile(profile:FounderProfile)->Result<(),Box<dyn Error>>{
  let serialization = serde_json::to_string_pretty(&profile);

  tokio::fs::write("hehejson.json",serialization);
}


match 