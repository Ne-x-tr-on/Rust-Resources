// vec![]
// Vec::new();


pub fn pushfn(){
  let mut test_int: Vec<i32> = Vec::new();
  test_int.push(30);
  test_int.push(30);
  test_int.push(30);
  test_int.push(30);
  test_int.push(30);
  println!("TestInt Value: {:?}\nLength: {:?}",test_int,test_int.len());

  println!("Specific Value: {:?}",test_int[0]);

  println!("Specific Range Value: {:?}\nAs Slice: {:?}",&(test_int).as_slice()[0..2],test_int.as_slice());
  
}

#[allow(unused)]
pub fn test_string(){
  let mut football_team:Vec<&str> = Vec::new();
  let mut team:Vec<&str>  = vec!["Liverpool","Mancity","RealMadrid","Arsenal"];

  team.push("Wolves");
  football_team.extend(team);
  
  // football_team.extend(team.clone());

  football_team.push("Barcelona");
  // team.push("Wolves");

  println!("This is the team: {:?}",football_team)
  
}

#[derive(Debug,Clone)]
struct TestCar{
  manufacturer:String,
  model:String,
}

