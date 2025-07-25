#[tokio::main]
#[allow(dead_code)]

async fn main(){
//   let x = getdata();
//   x.await;
let mut handles = vec![];

for _i in 0..2{
    let handle = tokio::spawn(async move{
        getdata().await;
    });
    handles.push(handle);
}

}

async fn getdata(){
    println!("Getting data");
    let s1 = readdata().await;
    println!("Data from {} has reflected",s1);
    let s2 = readdata().await;
    println!("Data from {} has reflected",s2);
}

async fn readdata() -> String {
    "DB Resuts".to_owned()
}