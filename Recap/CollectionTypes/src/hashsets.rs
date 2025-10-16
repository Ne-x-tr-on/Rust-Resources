use std::collections::HashSet;

pub fn test_hashsets(){
  let planet_list = HashSet::from(["Merury", "Earth"]);
  // println!("Thanks for adding {:?}",planet_list);
  let add_planets = HashSet::from(["Venus","Earth","Mars"]);

  // let planet_diff = planet_list.difference(&add_planets);
  //  println!("Thanks for adding {:?}",planet_diff);

  let planet_diff = planet_list.symmetric_difference(&add_planets);

  for planets in planet_diff{
     println!("You added {:#?}",planets);
  }

}