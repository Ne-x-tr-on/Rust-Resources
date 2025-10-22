fn main() {
    println!("=== 1️⃣ Creating Vectors ===");
    let v_empty: Vec<i32> = Vec::new();
    let mut v_init = vec![1, 2, 3, 4];
    println!("Empty vector: {:?}", v_empty);
    println!("Initialized vector: {:?}", v_init);

    println!("\n=== 2️⃣ Adding Elements ===");
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("Numbers after push: {:?}", numbers);

    println!("\n=== 3️⃣ Accessing Elements ===");
    println!("First element (index): {}", numbers[0]);
    match numbers.get(1) {
        Some(val) => println!("Second element (get): {}", val),
        None => println!("No element at index 1"),
    }

    println!("\n=== 4️⃣ Iterating Over Vectors ===");
    for val in &numbers {
        println!("Immutable iteration: {}", val);
    }
    for val in &mut numbers {
        *val += 5;
    }
    println!("After mutable iteration: {:?}", numbers);

    println!("\n=== 5️⃣ Removing Elements ===");
    let last = numbers.pop();
    println!("After pop: {:?}, popped: {:?}", numbers, last);
    numbers.push(50);
    let removed = numbers.remove(1);
    println!("After remove index 1: {:?}, removed: {}", numbers, removed);

    println!("\n=== 6️⃣ Vector Slices ===");
    let v_slice = &numbers[0..2];
    println!("Slice [0..2]: {:?}", v_slice);

    println!("\n=== 7️⃣ Capacity and Memory ===");
    let mut v_capacity = Vec::with_capacity(5);
    println!("Initial capacity: {}", v_capacity.capacity());
    v_capacity.push(1);
    v_capacity.push(2);
    println!("Capacity after 2 pushes: {}", v_capacity.capacity());
    println!("Length: {}", v_capacity.len());

    println!("\n=== 8️⃣ Concatenation and Extending ===");
    let mut v1 = vec![1, 2];
    let v2 = vec![3, 4];
    v1.extend(v2);
    println!("After extend: {:?}", v1);

    let mut v3 = vec![5,6];
    let mut v4 = vec![7,8];
    v3.append(&mut v4);
    println!("After append: {:?}, v4: {:?}", v3, v4);

    println!("\n=== 9️⃣ Advanced Operations ===");
    let mut v_sort = vec![3,1,4,2];
    v_sort.sort();
    println!("Sorted: {:?}", v_sort);
    v_sort.reverse();
    println!("Reversed: {:?}", v_sort);

    let mut v_retain = vec![1,2,3,4,5];
    v_retain.retain(|&x| x % 2 == 0);
    println!("After retain even: {:?}", v_retain);

    let v_map = vec![1,2,3];
    let doubled: Vec<_> = v_map.iter().map(|x| x*2).collect();
    println!("Doubled vector: {:?}", doubled);

    println!("\n=== 🔟 Converting Between Vec and Array ===");
    let arr = [1,2,3];
    let vec_from_arr = arr.to_vec();
    println!("Vec from array: {:?}", vec_from_arr);

    let vec_to_arr: [i32; 3] = vec![4,5,6].try_into().unwrap();
    println!("Array from vec: {:?}", vec_to_arr);

    println!("\n=== 1️⃣1️⃣ Ownership and Cloning ===");
    let v_original = vec![1,2,3];
    let v_moved = v_original; // v_original moved
    // println!("{:?}", v_original); // ❌ would panic
    let v_clone = v_moved.clone();
    println!("Moved vector: {:?}, Cloned vector: {:?}", v_moved, v_clone);

    println!("\n=== 1️⃣2️⃣ Iterators and Functional Style ===");
    let v_iter = vec![1,2,3,4,5];
    let sum: i32 = v_iter.iter().sum();
    println!("Sum: {}", sum);

    let evens: Vec<_> = v_iter.iter().filter(|&&x| x%2==0).collect();
    println!("Even numbers: {:?}", evens);
}
