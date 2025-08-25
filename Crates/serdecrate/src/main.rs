pub mod serdecrate;

use serde::{Deserialize,Serialize};
use serde_json::{to_string_pretty,from_str};

use crate::serdecrate::Dog;
use crate::serdecrate::DogOwner;

fn main(){
    // let owner1 = DogOwner{
    //     first_name:"Newton".to_string(),
    //     last_name:"Kamau".to_string(),
    // };

    // let dog01 = Dog{
    //     name:"Tommy".to_string(),
    //     year_born:2012,
    //     owner:owner1,
    // };

    // let dog_ser = to_string_pretty(&dog01);
    // if dog_ser.is_ok(){
    //     println!("{}",dog_ser.ok().unwrap());
    // } else{
    //     println!("{:?}",dog_ser.err());
    // }

    // // match dog_ser{
    // //     Ok(data) => println!("Ok: {}",data),
    // //     Err(e) => println!("Error in {}",e),
    // // }

    // let owner1 = DogOwner{
    //     first_name:"Newton".to_string(),
    //     last_name:"Kamau".to_string(),
    // };

fn deserialize(){

    let json_string: &str = r#"
    {
  "Name": "Tommy",
  "YearBorn": 2012,
  "Owner": {
    "FirstName": "Newton",
    "LastName": "Kamau"
  }
}
  "#;

     let dog_deser = from_str::<Dog>(json_string);
     match dog_deser{
        Ok(data) => println!("{:?}",data),
        Err(e) => println!("{:#?}",e),
     }
   }
}