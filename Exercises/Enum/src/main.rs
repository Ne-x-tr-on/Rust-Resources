#[warn(dead_code)]
fn main(){
    enum IpAddrKind{
        V4,
        V6,
    }

    struct IpAddr{
        kind:IpAddrKind,
        address:String,
    }

    let _four:IpAddrKind =  IpAddrKind::V4;
    let _six:IpAddrKind = IpAddrKind::V6;

    fn route(_address:IpAddrKind){

    }

    route(IpAddrKind::V4);


    let home :IpAddr = IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };
    println!("Home address: {}",home.address);

}