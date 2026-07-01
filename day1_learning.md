# Rust Day 1 Learning Notes

## Cargo & Command Line
- Checking tools:
  ```powershell
  rustc --version
  cargo --version
  ```
- Creating new project:
  ```powershell
  cargo new hello_rust
  ```
- Running the project:
  ```powershell
  cd hello_rust
  cargo run
  ```
  *(Note: Must be inside the directory containing Cargo.toml to run, otherwise it fails)*

---

## Variables and Mutability
- By default, all variables defined with `let` are immutable (read-only).
- Attempting to reassign them will cause a compilation error:
  ```rust
  let x = 10;
  x = 11; // Error: cannot assign twice to immutable variable
  ```
- To make a variable mutable, use `let mut`:
  ```rust
  let mut x = 10;
  x = 11; // Works
  ```

---

## Variable Declaration Syntax
- Assign using `=`.
- Avoid using `is` keyword (which is invalid).
  - Incorrect: `let name is = "Siddhant";`
  - Correct: `let name = "Siddhant";`

---

## Functions and Structure
- All execution logic must be wrapped inside a function.
- The entry point function is `main()`.
- You can write other functions and call them from `main()` to run them.
- Example structure:
  ```rust
  fn mains() {
      print!("Hello, world! Welcome to the world of Rust Siddhant ");
  }

  fn mainn(){
      let mut x = 10;
      print!("the value of x {}", x);
      x = 11;
      println!("the value of x {}", x);
  }

  fn main(){
      mains();
      mainn();

      let name = "Siddhant";
      let age = 23;
      print!("the name is {} and age is {}", name, age);
  }
  ```

---

## Printing
- `print!` - prints text without a newline at the end.
- `println!` - prints text and adds a newline at the end.
- String interpolation works by putting `{}` as placeholders and listing variables at the end.
  - Example: `print!("the name is {} and age is {}", name, age);`
