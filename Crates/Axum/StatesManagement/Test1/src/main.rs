use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::cell::RefCell;

fn main() {
    println!("Learning about Smart Pointers:\nBox|Rc|Arc|Mutex|RefCell|RWLock");
    // println!("");
    println!("1: Box in Rust!");
    let m = Box::new("Newton");
    println!("Value for Box :{}", m);

    println!("\n2: Working on Rc");
    let c = Rc::new("Value for Rc");
    let ca = Rc::clone(&c);
    let cb = Rc::clone(&c);
    println!("Rc value:{} | 1st Clone: {}| 2nd Clone: {}", c, ca, cb);

    println!("\n3: Working on Arc");
    let mut name = String::from("Newton");

    let data = Arc::new(name);
    let mut handles = vec![];

    for _ in 0..2{
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move ||{
            println!("Here is the shares Data: {}",data_clone);
        });
        handles.push(handle);
    }
    for data in handles{
        data.join().unwrap();
    }

    println!("\n3: Working on Arc<Mutex>");
    let car = Arc::new(Mutex::new(-100));

    let mut handles = vec![];

    for _ in 0..1000{
        let car_clone = Arc::clone(&car);
        let handle = thread::spawn(move||{
            let mut garage = car_clone.lock().unwrap();
            *garage+=1
        });
        handles.push(handle);
    }

    for cars in handles{
        cars.join().unwrap();
    }
    println!("The final Mileage of the Car is {}",*car.lock().unwrap());

    println!("\n4: Working on RefCell");
    let dodge = RefCell::new(0); //Mileage of the Car
    {
        let mut engine = dodge.borrow_mut();
        *engine+=1;
    }
    println!("Finale Milage of the car is {}",dodge.borrow());


    let car = Rc::new(RefCell::new(0)); // 0 mileage

    let friend1 = car.clone();
    let friend2 = car.clone();

    // friend1 modifies car
    {
        let mut engine = friend1.borrow_mut();  // mechanic opens hood
        *engine += 10;
    }

    // friend2 also modifies
    {
        let mut engine = friend2.borrow_mut();
        *engine += 5;
    }

    println!("Final mileage: {}", car.borrow());
}



