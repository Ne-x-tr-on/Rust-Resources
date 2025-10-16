use std::collections::HashMap;

// pub fn test_hashmaps(){
//   let mut score = HashMap::new();
//   score.insert(String::from("Newton"),50);
//   println!("Student Score:\n{:?}",score);

// }

// let stock_list:HashMap<String,f32> = HashMap::new();
pub fn test_hashmaps(){
let mut stock_list = HashMap::<String,f64>::new();

println!("{}",stock_list.len());
// println!("{}",stock_list.is_empty());
stock_list.insert("NVDA".to_string(), 478.82);
stock_list.insert("NEZA".to_string(), 400.82);
// println!("{}",stock_list.len());
stock_list.insert("Teffarm".to_string(), 500.82);
stock_list.insert("GLOHA".to_string(), 500.82);

println!("{:#?}",stock_list);
// pub fn remove_stock(|| stock_list){
//   // stock_list
// }

println!("Removed a particular stock");
stock_list.remove(&String::from("NEZA"));
println!("{:#?}",stock_list);


for (stock,value) in &stock_list{
  // &stock_list.entry(String::from("Lydatron")).or_insert(500.00);
  // stock_list.insert("BEN".to_string(),900.00);
  println!("Stock: {} - Running At: {}",stock,value);
}
}