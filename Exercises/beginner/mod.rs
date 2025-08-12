// ========================
// RUST MODULE PRACTICE QUESTIONS (No Answers)
// ========================

// Q1: Create a module named `math_utils` with a public function `add(a, b)` that returns their sum. Call this function from `main()`.

// Q2: Create a module `printer` with a private function `greet()`. Try calling it from `main()` and observe what happens.

// Q3: Inside a module `auth`, define a `pub fn login()` and `pub fn logout()`. Call both from `main()`.

// Q4: Create a module `hello` in a separate file `hello.rs`, and define a `pub fn say_hello()`. Import and call it from `main.rs`.

// Q5: Nest a module `geometry` inside a module `shapes` and define `pub fn area()`. Call `area()` from `main()`.

// Q6: Use the `use` keyword to shorten the path for calling `shapes::geometry::area()`.

// Q7: Create a module `converter` with a public constant `PI`. Access and print it from `main()`.

// Q8: In a module `tools`, define a struct `Tool` with a `pub fn use_tool()` method. Instantiate and use it from `main()`.

// Q9: Write a module `greetings` with a function `hello()` that returns a `String`. Print its value in `main()`.

// Q10: Define a module `basic` inside `main.rs`. Inside it, define a function `pub fn explain()`. Call `basic::explain()` in `main()`.

// Q11: Write a nested module structure `outer::middle::inner` and define a public function `shout()` inside `inner`. Call it from `main()`.

// Q12: Create two modules `a` and `b`, each with a `pub fn info()`. Call both from `main()` using full paths.

// Q13: Use `pub(crate)` to restrict access to a function `debug()` so it's only visible within the crate.

// Q14: Define a function inside a module that uses `super::` to access a parent module's function.

// Q15: Create a module with a `mod tests` block and write a unit test function `#[test] fn test_add()`.

// Q16: In a module `config`, define a struct with some private and some public fields. Access only the public fields in `main()`.

// Q17: Build a `file-based` module project with `main.rs`, `utils.rs`, and `db.rs`. Import and call a function from both modules in `main.rs`.

// Q18: Use `mod` to create a module that contains another module defined in a separate file (`submod.rs` inside `parent.rs`).

// Q19: Write a module `maths` with submodules `basic` and `advanced`, each with their own functions. Call functions from both in `main()`.

// Q20: Simulate a simple library layout using `lib.rs` and organize at least 2 modules inside it, calling them from `main.rs`.