// Practice 1: Hello World (Prints a welcome message)
/*
fn main() {
    println!("Hello, world! Welcome to the world of Rust Siddhant ");
}
*/

// Practice 2: Variable Mutability Error
// Fails to compile because variables are immutable by default in Rust.
/*
fn main() {
    let x = 10;
    println!("the value of x {}", x);
    x = 11; // Compile error: cannot assign twice to immutable variable
    println!("the value of x {}", x);
}
*/

// Practice 3: Syntax issues with global scope & 'is' keyword
// 1. You cannot write variable declarations/macros at global scope
// 2. You cannot use 'is'; Rust uses '=' for variable assignment
/*
let name is = "Siddhant";
let age is = 23;
println!("the name is {} and age is {}", name, age);
*/

// Final Working Code:
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

