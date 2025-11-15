use std::thread;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::Arc,
};

struct Owner {
    name: String,
    tools: Vec<Weak<Tool>>,
}

struct Tool {
    owner: Rc<Owner>,
}

fn arcfn() {
    let call = Arc::new("David Kamau");

    let mut handles = vec![];

    for _ in 0..5 {
        let data = Arc::clone(&call);
        handles.push(thread::spawn(move || {
            println!("Here they are: {}", data);
        }));
    }
    for h in handles{
        h.join().unwrap();
    }
}

fn main() {
    // println!("Working with Rc");

    let Developer = Rc::from(Owner {
        name: "David".to_string(),
        tools: vec![],
    });
    let pliers = Tool {
        owner: Rc::clone(&Developer),
    };
    let wrench = Tool {
        owner: Developer.clone(),
    };
    arcfn();
}
