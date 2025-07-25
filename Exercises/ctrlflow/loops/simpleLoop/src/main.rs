fn main() {
    let mut outer_count = 0;

    'outer: loop {
        let mut inner_count = 0;

        loop {
            if inner_count == 3 {
                break; // breaks the inner loop only
            }
            if outer_count == 2 {
                break 'outer; // breaks the outer loop using the label
            }

            inner_count += 1;
        }

        outer_count += 1;
    }

    println!("outer_count = {}", outer_count); // Prints 2
}
