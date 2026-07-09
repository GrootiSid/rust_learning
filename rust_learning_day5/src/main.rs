// day 5: learning variables, mutability, constants & shadowing

// 3. declaring constants (requires explicit type annotation and UPPER_SNAKE_CASE)
const PI: f64 = 3.14159;
const MAX_USERS: u32 = 500;
const APP_NAME: &str = "Siddhant's Rust App";

fn main() {
    // 1. storing and printing name, age, and city
    let name = "Siddhant";
    let age = 22;
    let city = "Bengaluru";
    println!("User Info -> Name: {}, Age: {}, City: {}", name, age, city);

    // 2. mutable variable demonstration (uses 'mut')
    let mut salary = 50000;
    println!("Starting Salary: {}", salary);
    salary = 60000; // changing the value
    println!("Updated Salary: {}", salary);

    // printing constants to verify they work
    println!(
        "Constants: App = {}, Max Users = {}, Pi = {}",
        APP_NAME, MAX_USERS, PI
    );

    // 4. shadowing demonstration
    // we can reuse the same name 'spaces' to change its value and type
    let spaces = "   "; // string containing spaces
    let spaces = spaces.len(); // shadows 'spaces' to an integer representing count
    println!("Number of spaces: {}", spaces);
}
