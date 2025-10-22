# Rust Vec<T> Overview

This Rust program demonstrates the essential features of the `Vec<T>` type (vectors), which are growable, heap-allocated collections of elements of the same type.

## What the Code Covers

1. **Creating Vectors**
   - `Vec::new()` to create an empty vector
   - `vec![]` macro for initializing with elements

2. **Adding Elements**
   - `push()` to add elements to the end

3. **Accessing Elements**
   - Indexing with `[]` (panics if out of bounds)
   - `get()` method (returns `Option` for safe access)

4. **Iterating Over Vectors**
   - Immutable iteration
   - Mutable iteration (modifying elements)

5. **Removing Elements**
   - `pop()` to remove the last element
   - `remove(index)` to remove an element by index

6. **Vector Slices**
   - Borrowing part of a vector with `&v[start..end]`

7. **Capacity and Memory**
   - Using `with_capacity()`, `len()`, and `capacity()`

8. **Concatenation and Extending**
   - `extend()` to append elements from another vector
   - `append()` to move elements from another vector

9. **Advanced Operations**
   - `sort()` and `reverse()`
   - `retain()` for conditional filtering
   - Mapping with iterators (`map()`)

10. **Converting Between Vec and Array**
    - `.to_vec()` for array → vector
    - `.try_into()` for vector → array

11. **Ownership and Cloning**
    - Understanding move semantics
    - Cloning vectors with `.clone()`

12. **Iterators and Functional Style**
    - Summing elements with `iter().sum()`
    - Filtering elements with `iter().filter().collect()`

## How to Run

1. Save the code in a file, e.g., `vec_overview.rs`.
2. Compile and run using Rust:
   ```bash
   rustc vec_overview.rs
   ./vec_overview
