// ========================
// RUST TUPLE PRACTICE QUESTIONS & ANSWERS
// ========================

// Q1
fn q1() {
    let person = ("Newton", 18u8, 3.7f32);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("GPA: {}", person.2);
}

// Q2
fn q2() {
    let (a, b, c) = (100, 3.14, 'A');
    println!("{a}, {b}, {c}");
}

// Q3
fn q3() {
    let data = ("Rust", 2025);
    println!("Year: {}", data.1);
}

// Q4
fn get_coordinates() -> (i32, i32) {
    (5, 10)
}

fn q4() {
    let (x, y) = get_coordinates();
    println!("Coordinates: ({}, {})", x, y);
}

// Q5
fn q5() {
    let mut stats = (10, 5.5);
    stats.0 = 15;
    println!("{:?}", stats);
}

// Q6
fn q6() {
    let mut data = (String::from("rust"), 1);
    data.0 = data.0.to_uppercase();
    println!("{:?}", data);
}

// Q7
fn q7() {
    let tuple = (42, (3.14, 'z'));
    println!("Inner char: {}", (tuple.1).1);
}

// Q8
fn split_name(full: &str) -> (&str, &str) {
    let parts: Vec<&str> = full.split(' ').collect();
    (parts[0], parts[1])
}

fn q8() {
    let (first, last) = split_name("Newton Kamau");
    println!("First: {}, Last: {}", first, last);
}

// Q9
fn q9() {
    let name = "Rust";
    let info = (name, 10u8);
    println!("Name: {}, Value: {}", info.0, info.1);
}

// Q10
fn q10() {
    let pairs = [(1, 2), (3, 4), (5, 6)];
    for (a, b) in pairs.iter() {
        println!("Sum: {}", a + b);
    }
}

// Q11
fn calc_ops(a: i32, b: i32) -> (i32, i32, i32) {
    (a + b, a - b, a * b)
}

fn q11() {
    let result = calc_ops(10, 5);
    println!("Sum: {}, Diff: {}, Product: {}", result.0, result.1, result.2);
}

// Q12
fn q12(input: (i32, bool)) {
    if input.1 && input.0 > 0 {
        println!("Active");
    }
}

// Q13
fn q13() -> Vec<f64> {
    let coords = [(1, 2), (3, 4), (5, 6)];
    coords.iter().map(|(x, y)| ((*x as f64).powi(2) + (*y as f64).powi(2)).sqrt()).collect()
}

// Q14
fn q14() {
    let tup = (1, ('A', 3.14));
    let (x, (ch, val)) = tup;
    println!("{} {} {}", x, ch, val);
}

// Q15
fn q15(t: (i32, i32)) {
    match t {
        (0, 0) => println!("Both zero"),
        _ => println!("Not both zero"),
    }
}

// Q16
fn q16(input: (i32, f32)) -> String {
    format!("Value: {}, Score: {:.1}", input.0, input.1)
}

// Q17
fn q17() {
    let people = vec![("Alice", 20), ("Bob", 17), ("Charlie", 25)];
    for (name, age) in people {
        if age > 18 {
            println!("{}", name);
        }
    }
}

// Q18
fn q18() {
    let t1 = (10, 20);
    let t2 = (10, 20);
    println!("Equal? {}", t1 == t2);
}

// Q19
fn q19() {
    let t = (10, 98.5);
    println!("Value: {}, Score: {:.1}", t.0, t.1);
}

// Q20
fn q20() {
    let mut a = (10, 1.1);
    let mut b = (20, 2.2);
    std::mem::swap(&mut a.0, &mut b.0);
    println!("a: {:?}, b: {:?}", a, b);
}

fn main() {
    q1(); q2(); q3(); q4(); q5(); q6(); q7(); q8(); q9(); q10();
    q11(); q12((5, true));
    println!("Distances: {:?}", q13());
    q14(); q15((0, 0));
    println!("{}", q16((42, 88.9)));
    q17(); q18(); q19(); q20();
}
