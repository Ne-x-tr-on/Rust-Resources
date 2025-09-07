// pub mod serdecrate;
pub mod simply;

use crate::simply::Dog;
// use crate::serdecrate::Dog;
// use crate::serdecrate::DogOwner;

// fn main(){
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

// fn deserialize(){

//     let json_string: &str = r#"
//     {
//   "Name": "Tommy",
//   "YearBorn": 2012,
//   "Owner": {
//     "FirstName": "Newton",
//     "LastName": "Kamau"
//   }
// }
//   "#;

  //    let dog_deser = from_str::<Dog>(json_string);
  //    match dog_deser{
  //       Ok(data) => println!("{:?}",data),
  //       Err(e) => println!("{:#?}",e),
  //    }
  //  }
// }

fn main() {
    // JSON string (coming from file, API, etc.)
    let json_str = r#"
        {
            "name": "Tommy",
            "age": 3
        }
    "#;

    // 1️⃣ Deserialize JSON → Rust struct
    let dog: Dog = serde_json::from_str(json_str).unwrap();
    println!("Deserialized: {:?}", dog);

    // 2️⃣ Serialize Rust struct → JSON
    let dog_json = serde_json::to_string(&dog).unwrap();
    println!("Serialized: {}", dog_json);


    // let dog_deser = from_str::<Dog>(json_string);
    //  match dog_deser{
    //     Ok(data) => println!("{:?}",data),
    //     Err(e) => println!("{:#?}",e),
    //  }
}


