enum Newton{
    name,
    gender,
    vision,
    title,
}


#[Debug]
fn main(){
    let _name = Newton::name;
    println!("Hey {:?}",_name);
}