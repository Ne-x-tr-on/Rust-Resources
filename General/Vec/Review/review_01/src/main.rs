fn main() {
    let mut v :Vec<i32> = Vec::new();
    v.push(11);
    v.push(12);
    v.push(13);
    v.push(14);
    v.push(15);
    // println!("Value vec is {:?}",v[3])

    // Using the Get Key Words
    match v.get(0){
        Some(v) => println!("Some from Get:\n{:?}",v),
        None => println!("No element at this index"),
    }

    // using .pop() to remove the last Element and returns Option<T>
    println!("All values:\n{:?}",v);
    let last = v.pop();
    println!("Pop Value:{:?}\nThe rest:\n{:?}",last,v);

    // Remove Value By Index using the Remove Key World
    let removed = v.remove(1);
    println!("{:?}, removed: {}",v,removed);

    // Iteration in Vectors
    for _values in &mut v{
        // println!("Iteration Values:\n{:?}",_values);
    }

    for val in &mut v {
        *val += 10; // Need * to dereference
    }
    // println!("{:?}", v); 



}
