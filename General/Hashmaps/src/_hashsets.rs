use std::collections::HashSet;

pub fn test_hashset_basic(){
  let mut planet_list = HashSet::from
  (["Mercury","Venus","Mars","Earth"]);
  
  let planet_list_more = HashSet::from(["Earth","Jupiter","Saturn","Mercry"]);
    // for planets in planet_diff{
  //   println!("Thankyou for Adding: {:#?}",planets);
  // }

// Normal Difference
  // planet_list.difference(&planet_list_more);
  let planet_diff = planet_list.difference(&planet_list_more);

  // for planet in planet_list{
  // println!("Thankyou for adding {:#}",planet);
  // }
planet_list.insert("Pluto");

  // Symetric Difference
  //  planet_list.symmetric_difference(&planet_list_more);
  let planet_symdiff = planet_list.symmetric_difference(&planet_list_more);
  
  
  for planets in planet_symdiff{
    println!("Thankyou for Adding: {:#?}",planets);
  }

  
}

