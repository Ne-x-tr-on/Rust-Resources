// Enums | enumerations - 
// allow you to define a type by enumerating its possible variants. 

// enum IpAddrKind{
//     v4,
//     v6,
// }

// struct IpAddr {
//      kind:IpAddrKind,
//     address:String,
// }


enum IpAddr{
    v4(u8,u8,u8,u8),
    v6(String),
}



fn main(){

// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;

// route(IpAddrKind::v4);
// route(IpAddrKind::v6);

//         let home = IpAddr {
//         kind:IpAddrKind::v4,
//         address:String::from("127.0.0.1"),
//   };

//     let home = IpAddrKind::v4(String::from("127.0.0.1"));
//     let loopback = IpAddrKind::V6(String::from("::1"));

// fn route(_ip_kind:IpAddrKind){}

let home = IpAddr::v4(127,0,0,1);
let loopback = IpAddr::v6(String::from("::1"));

}

