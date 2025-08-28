use std::collections::HashMap;

pub fn test_hashmap_basic(){
  let mut stock_list:HashMap<String,f32> = HashMap::new();

  stock_list.insert("NVDA".to_string(),478.52);
  stock_list.insert("AAPL".to_string(),478.52);
  stock_list.insert("AMSC".to_string(),50.52);

  stock_list.entry("META".to_string()).or_insert(346.60);

  stock_list.remove(&("AAPL".to_string()));

  for (ticker,current_value) in &stock_list{
    println!("{:#?}: {:#?}",ticker,current_value);
  }


  println!("{:?}",stock_list.len());
  println!("{:?}",stock_list.is_empty());
  println!("{:#?}",stock_list);
}