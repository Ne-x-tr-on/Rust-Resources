pub mod recap;

fn main() {
    println!("HTTP Recap Playground 🚀");

    // Blocking CRUD
    if let Err(e) = recap::blocking::get_example() {
        eprintln!("Error in GET: {:#?}", e);
    }
    if let Err(e) = recap::blocking::post_example() {
        eprintln!("Error in POST: {:#?}", e);
    }
    if let Err(e) = recap::blocking::put_example() {
        eprintln!("Error in PUT: {:#?}", e);
    }
    if let Err(e) = recap::blocking::delete_example() {
        eprintln!("Error in DELETE: {:#?}", e);
    }
}
