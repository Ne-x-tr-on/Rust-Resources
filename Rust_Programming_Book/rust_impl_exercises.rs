
// ✅ 1. Start with the Official Docs
// Axum has some of the best Rust framework docs out there:

// 📘 Axum Official Docs

// 🔗 Axum Examples GitHub Repo

// Tip: Don’t just read — run each example, modify it, break it, and rebuild it.

// ✅ 2. Use This Learning Roadmap
// Here’s a path to self-teach Axum effectively:

// 🔹 Week 1-2: Foundations
// ✅ Rust async/await with tokio

// ✅ Understand Router, handler, extract, response, tower

// ✅ Start a basic Axum app (e.g., “Hello vending machine”)

// 🔹 Week 3-4: Build REST APIs
// ✅ Create endpoints (GET, POST, PUT, DELETE)

// ✅ Learn routing and extractors

// ✅ Connect to a PostgreSQL/SQLite DB using sqlx or sea-orm

// ✅ Error handling and middleware

// 🔹 Week 5+: Real Projects
// Build a mock vending dashboard

// Add user login/register (JWT) with axum-extra

// Add facial recognition endpoint that connects to a Python model

// Add real-time communication via WebSockets (e.g. for device status)

// ✅ 3. Study Real Projects (Open Source)
// There are already Axum-based projects on GitHub:

// 🔍 Search: axum api backend or axum jwt crud

// Clone, run, and reverse-engineer how they work

// Examples:

// https://github.com/tokio-rs/axum/tree/main/examples

// https://github.com/launchbadge/sqlx/tree/main/examples/axum

// ✅ 4. Pair with Crates You’ll Need
// For ML, databases, auth, file uploads, use these:

// Goal	Crate
// Async runtime	tokio
// JSON APIs	serde, serde_json
// Database	sqlx, sea-orm
// Auth (JWT)	jsonwebtoken
// Facial recog service	Call external Python API via reqwest
// WebSockets	axum::extract::ws
// Middleware/logging	tower-http






// 1. 📂 Variables and Ownership
// ❓ Create an immutable variable and print its value.

// 2. 📂 Variables and Ownership
// ❓ Create a mutable variable and change its value.

// 3. 📂 Variables and Ownership
// ❓ Move ownership of a String to a function.

// 4. 📂 Variables and Ownership
// ❓ Use borrowing with & to avoid moving ownership.

// 5. 📂 Variables and Ownership
// ❓ Use mutable borrowing with &mut to modify a value in a function.

// 6. 📂 Functions
// ❓ Write a function that returns the sum of two integers.

// 7. 📂 Functions
// ❓ Write a function that returns a boolean if a number is even.

// 8. 📂 Functions
// ❓ Call a function from inside another function.

// 9. 📂 Functions
// ❓ Use a function that returns a tuple.

// 10. 📂 Functions
// ❓ Define a function that returns nothing (unit type).

// 11. 📂 Structs
// ❓ Define a struct named `Book` with title and pages.

// 12. 📂 Structs
// ❓ Create an instance of the `Book` struct.

// 13. 📂 Structs
// ❓ Access fields of a struct instance.

// 14. 📂 Structs
// ❓ Use struct update syntax to create a second book.

// 15. 📂 Structs
// ❓ Create a tuple struct with 3 values.

// 16. 📂 Impl Blocks
// ❓ Implement a method `summary` for a struct `Article`.

// 17. 📂 Impl Blocks
// ❓ Create an `impl` block with a constructor method `new()`.

// 18. 📂 Impl Blocks
// ❓ Use &self and &mut self in struct methods.

// 19. 📂 Impl Blocks
// ❓ Create multiple methods inside an impl block.

// 20. 📂 Impl Blocks
// ❓ Call a method on a struct instance.

// 21. 📂 Enums and Pattern Matching
// ❓ Create an enum `Message` with 3 variants.

// 22. 📂 Enums and Pattern Matching
// ❓ Use a match statement to handle enum values.

// 23. 📂 Enums and Pattern Matching
// ❓ Use `if let` to match a single enum variant.

// 24. 📂 Enums and Pattern Matching
// ❓ Create an enum with data in its variants.

// 25. 📂 Enums and Pattern Matching
// ❓ Call a method on an enum using an impl block.

// 26. 📂 Traits
// ❓ Define a trait called `Printable` with one method.

// 27. 📂 Traits
// ❓ Implement a trait for a struct.

// 28. 📂 Traits
// ❓ Use trait bounds in a generic function.

// 29. 📂 Traits
// ❓ Use `dyn` keyword for dynamic dispatch.

// 30. 📂 Traits
// ❓ Implement Debug trait manually using `fmt`.

// 31. 📂 Ownership & Borrowing Advanced
// ❓ Show an example where ownership is moved and causes error.

// 32. 📂 Ownership & Borrowing Advanced
// ❓ Fix ownership issues using borrowing.

// 33. 📂 Ownership & Borrowing Advanced
// ❓ Use clone() to copy a String.

// 34. 📂 Ownership & Borrowing Advanced
// ❓ Demonstrate dangling reference and fix it.

// 35. 📂 Ownership & Borrowing Advanced
// ❓ Pass ownership to a function and return it back.

// 36. 📂 Variables and Ownership
// ❓ Create an immutable variable and print its value. (Practice Again)

// 37. 📂 Variables and Ownership
// ❓ Create a mutable variable and change its value. (Practice Again)

// 38. 📂 Variables and Ownership
// ❓ Move ownership of a String to a function. (Practice Again)

// 39. 📂 Variables and Ownership
// ❓ Use borrowing with & to avoid moving ownership. (Practice Again)

// 40. 📂 Variables and Ownership
// ❓ Use mutable borrowing with &mut to modify a value in a function. (Practice Again)

// 41. 📂 Functions
// ❓ Write a function that returns the sum of two integers. (Practice Again)

// 42. 📂 Functions
// ❓ Write a function that returns a boolean if a number is even. (Practice Again)

// 43. 📂 Functions
// ❓ Call a function from inside another function. (Practice Again)

// 44. 📂 Functions
// ❓ Use a function that returns a tuple. (Practice Again)

// 45. 📂 Functions
// ❓ Define a function that returns nothing (unit type). (Practice Again)

// 46. 📂 Structs
// ❓ Define a struct named `Book` with title and pages. (Practice Again)

// 47. 📂 Structs
// ❓ Create an instance of the `Book` struct. (Practice Again)

// 48. 📂 Structs
// ❓ Access fields of a struct instance. (Practice Again)

// 49. 📂 Structs
// ❓ Use struct update syntax to create a second book. (Practice Again)

// 50. 📂 Structs
// ❓ Create a tuple struct with 3 values. (Practice Again)

// 51. 📂 Impl Blocks
// ❓ Implement a method `summary` for a struct `Article`. (Practice Again)

// 52. 📂 Impl Blocks
// ❓ Create an `impl` block with a constructor method `new()`. (Practice Again)

// 53. 📂 Impl Blocks
// ❓ Use &self and &mut self in struct methods. (Practice Again)

// 54. 📂 Impl Blocks
// ❓ Create multiple methods inside an impl block. (Practice Again)

// 55. 📂 Impl Blocks
// ❓ Call a method on a struct instance. (Practice Again)

// 56. 📂 Enums and Pattern Matching
// ❓ Create an enum `Message` with 3 variants. (Practice Again)

// 57. 📂 Enums and Pattern Matching
// ❓ Use a match statement to handle enum values. (Practice Again)

// 58. 📂 Enums and Pattern Matching
// ❓ Use `if let` to match a single enum variant. (Practice Again)

// 59. 📂 Enums and Pattern Matching
// ❓ Create an enum with data in its variants. (Practice Again)

// 60. 📂 Enums and Pattern Matching
// ❓ Call a method on an enum using an impl block. (Practice Again)

// 61. 📂 Traits
// ❓ Define a trait called `Printable` with one method. (Practice Again)

// 62. 📂 Traits
// ❓ Implement a trait for a struct. (Practice Again)

// 63. 📂 Traits
// ❓ Use trait bounds in a generic function. (Practice Again)

// 64. 📂 Traits
// ❓ Use `dyn` keyword for dynamic dispatch. (Practice Again)

// 65. 📂 Traits
// ❓ Implement Debug trait manually using `fmt`. (Practice Again)

// 66. 📂 Ownership & Borrowing Advanced
// ❓ Show an example where ownership is moved and causes error. (Practice Again)

// 67. 📂 Ownership & Borrowing Advanced
// ❓ Fix ownership issues using borrowing. (Practice Again)

// 68. 📂 Ownership & Borrowing Advanced
// ❓ Use clone() to copy a String. (Practice Again)

// 69. 📂 Ownership & Borrowing Advanced
// ❓ Demonstrate dangling reference and fix it. (Practice Again)

// 70. 📂 Ownership & Borrowing Advanced
// ❓ Pass ownership to a function and return it back. (Practice Again)

// 71. 📂 Variables and Ownership
// ❓ Create an immutable variable and print its value. (Practice Again)

// 72. 📂 Variables and Ownership
// ❓ Create a mutable variable and change its value. (Practice Again)

// 73. 📂 Variables and Ownership
// ❓ Move ownership of a String to a function. (Practice Again)

// 74. 📂 Variables and Ownership
// ❓ Use borrowing with & to avoid moving ownership. (Practice Again)

// 75. 📂 Variables and Ownership
// ❓ Use mutable borrowing with &mut to modify a value in a function. (Practice Again)

// 76. 📂 Functions
// ❓ Write a function that returns the sum of two integers. (Practice Again)

// 77. 📂 Functions
// ❓ Write a function that returns a boolean if a number is even. (Practice Again)

// 78. 📂 Functions
// ❓ Call a function from inside another function. (Practice Again)

// 79. 📂 Functions
// ❓ Use a function that returns a tuple. (Practice Again)

// 80. 📂 Functions
// ❓ Define a function that returns nothing (unit type). (Practice Again)

// 81. 📂 Structs
// ❓ Define a struct named `Book` with title and pages. (Practice Again)

// 82. 📂 Structs
// ❓ Create an instance of the `Book` struct. (Practice Again)

// 83. 📂 Structs
// ❓ Access fields of a struct instance. (Practice Again)

// 84. 📂 Structs
// ❓ Use struct update syntax to create a second book. (Practice Again)

// 85. 📂 Structs
// ❓ Create a tuple struct with 3 values. (Practice Again)

// 86. 📂 Impl Blocks
// ❓ Implement a method `summary` for a struct `Article`. (Practice Again)

// 87. 📂 Impl Blocks
// ❓ Create an `impl` block with a constructor method `new()`. (Practice Again)

// 88. 📂 Impl Blocks
// ❓ Use &self and &mut self in struct methods. (Practice Again)

// 89. 📂 Impl Blocks
// ❓ Create multiple methods inside an impl block. (Practice Again)

// 90. 📂 Impl Blocks
// ❓ Call a method on a struct instance. (Practice Again)

// 91. 📂 Enums and Pattern Matching
// ❓ Create an enum `Message` with 3 variants. (Practice Again)

// 92. 📂 Enums and Pattern Matching
// ❓ Use a match statement to handle enum values. (Practice Again)

// 93. 📂 Enums and Pattern Matching
// ❓ Use `if let` to match a single enum variant. (Practice Again)

// 94. 📂 Enums and Pattern Matching
// ❓ Create an enum with data in its variants. (Practice Again)

// 95. 📂 Enums and Pattern Matching
// ❓ Call a method on an enum using an impl block. (Practice Again)

// 96. 📂 Traits
// ❓ Define a trait called `Printable` with one method. (Practice Again)

// 97. 📂 Traits
// ❓ Implement a trait for a struct. (Practice Again)

// 98. 📂 Traits
// ❓ Use trait bounds in a generic function. (Practice Again)

// 99. 📂 Traits
// ❓ Use `dyn` keyword for dynamic dispatch. (Practice Again)

// 100. 📂 Traits
// ❓ Implement Debug trait manually using `fmt`. (Practice Again)

