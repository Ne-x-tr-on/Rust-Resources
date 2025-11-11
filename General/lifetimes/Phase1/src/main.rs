// fn main() {
//     // println!("Hello, world!");
//     let x =5;
//     // {
//     //     let x:i32 = 5;
//     //     r = &x;
//     // }
//     let r = &x;
//     println!("r: {}",r);
// }


// fn main(){
//     let string1 = String::from("Newton");
//   {
//     let string2 = String::from("Kamau");
//     let result = longest(string1.as_str(),string2.as_str());
//     println!("Longest String: {}",result);
//   }
// }

// fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
//     if x.len()< y.len(){
//         y
//     }
//     else{
//         x
//     }
// }

struct ImportantExcerpt<'a>{
    part:&'a str,
}

fn main(){
    let novel = String::from("Call me Ishmael. Some Years ago...");
    let first_sentence = novel.split('.').next().expect("Failed to read the novel");
    let i = ImportantExcerpt{
        part:first_sentence,
    };
}
