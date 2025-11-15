use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use std::thread;

fn main() {
    // 1️⃣ Box<T> - Your toy in a box
    let my_toy = Box::new("Toy Car");
    println!("Box toy: {}", my_toy); // Only one owner

    // 2️⃣ Rc<T> - Shared book in one room
    let book = Rc::new("Story Book");
    let friend1 = Rc::clone(&book);
    let friend2 = Rc::clone(&book);
    println!("Book shared: {}, {}, {}", book, friend1, friend2);
    // Many friends can read, no one can write

    // 3️⃣ Arc<T> - Shared book across houses (threads)
    let arc_book = Arc::new("Magic Book");
    let mut handles = vec![];
    for _ in 0..3 {
        let arc_clone = Arc::clone(&arc_book);
        handles.push(thread::spawn(move || {
            println!("Arc book read in thread: {}", arc_clone);
        }));
    }
    for h in handles { h.join().unwrap(); }

    // 4️⃣ RefCell<T> - Magic toy you can change inside the box
    let magic_box = RefCell::new(10);
    *magic_box.borrow_mut() += 5; // change value inside
    println!("Magic Box value: {}", magic_box.borrow());

    // 5️⃣ Mutex<T> - Cookie jar, one person at a time
    let cookie_jar = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..3 {
        let jar_clone = Arc::clone(&cookie_jar);
        handles.push(thread::spawn(move || {
            let mut cookies = jar_clone.lock().unwrap();
            *cookies += 1;
            println!("Cookie count: {}", cookies);
        }));
    }
    for h in handles { h.join().unwrap(); }

    // 6️⃣ RwLock<T> - Storybook, many readers or one writer
    let story_book = Arc::new(RwLock::new(String::from("Once upon a time")));
    {
        let r1 = story_book.read().unwrap();
        let r2 = story_book.read().unwrap();
        println!("Readers read: {}, {}", r1, r2);
    }
    {
        let mut w = story_book.write().unwrap();
        w.push_str(", and they lived happily.");
        println!("Writer updated story: {}", w);
    }
}
