// enum Unit{
//     MathsEng,
//     Autocad,
//     MechanicalScience,
    
//   }

// fn main(){

//   let _cad = Unit::Autocad;
//   let _maths = Unit::MathsEng;
//   let _science = Unit::MechanicalScience;

// }

enum Units{
  MathsEng(String),
  Digital_Literacy(String),
  WorkshopTechnology(String),
}

struct myUnits{
  types:Units,
}

let unit = myUnits