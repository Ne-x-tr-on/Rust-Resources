fn main(){

  enum IpAddrKind{
      V4,
      V6,
  }
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  fn route(_ip_kind:IpAddrKind){}

  route(_ip_kind:IpAddrKind::V4);
  route(_ip_kind:IpAddrKind::V6);

  struct IpAddr{
    kind:IpAddrKind,
    address:String,
  }

}