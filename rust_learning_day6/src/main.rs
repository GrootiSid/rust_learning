// Day 6: Rust Data Types (The Foundation of Memory)
// Practicing scalar types, compound types, type annotations, and inference.

fn main() {
    println!("--- Question 1: Storing Personal Info ---");
    // 1. Store your name, age, height, and login status using appropriate data types.
    let name: &str = "Siddhant";
    let age: u8 = 22; // age cannot be negative, and fits comfortably in 0-255 (u8)
    let height: f64 = 5.9; // height has decimals, f64 is the Rust default/recommended float
    let is_logged_in: bool = true; // binary state (logged in or not)

    println!("Name: {}", name);
    println!("Age: {} years", age);
    println!("Height: {} feet", height);
    println!("Logged In: {}", is_logged_in);
    println!();

    println!("--- Question 2: Scalar Types Demonstration ---");
    // 2. Create one variable of each scalar type (i32, u32, f64, bool, char) and print them.
    let integer_signed: i32 = -42; // signed integer can be negative
    let integer_unsigned: u32 = 100; // unsigned integer can only be positive
    let float_val: f64 = 3.14159; // floating point decimal
    let boolean_val: bool = false; // boolean
    let char_val: char = '🦀'; // single unicode character (emoji!)

    println!("i32 (Signed): {}", integer_signed);
    println!("u32 (Unsigned): {}", integer_unsigned);
    println!("f64 (Float): {}", float_val);
    println!("bool (Boolean): {}", boolean_val);
    println!("char (Character): {}", char_val);
    println!();

    println!("--- Question 3: Tuple containing Name, Age, and CGPA ---");
    // 3. Create a tuple containing your name, age, and CGPA, then print each value individually.
    let student_tuple: (&str, u8, f64) = ("Siddhant", 22, 9.5);
    
    // Accessing tuple elements using dot notation (0-indexed)
    println!("Tuple Name: {}", student_tuple.0);
    println!("Tuple Age: {}", student_tuple.1);
    println!("Tuple CGPA: {}", student_tuple.2);
    println!();

    println!("--- Question 4: Array of Five Marks ---");
    // 4. Create an array of five marks and print the first and last elements.
    let marks: [u32; 5] = [95, 90, 85, 99, 80]; // Array of 5 unsigned 32-bit integers

    println!("Marks Array: {:?}", marks);
    println!("First Element (Index 0): {}", marks[0]);
    println!("Last Element (Index 4): {}", marks[4]);
    println!();

    println!("--- Question 5: Type Annotation vs. Type Inference ---");
    // 5. Experiment with explicit type annotations and compare them with type inference.
    
    // Type Inference: Compiler determines type automatically
    let inferred_int = 500; // Defaults to i32
    let inferred_float = 99.9; // Defaults to f64
    let inferred_char = 'A'; // Defaults to char
    
    // Type Annotation: We explicitly specify the type
    let annotated_int: i16 = 500; // We force it to be a 16-bit signed integer
    let annotated_float: f32 = 99.9; // We force f32 instead of f64
    let annotated_char: char = 'A'; // Explicit char definition

    println!("Inferred Int (i32 default): {}", inferred_int);
    println!("Annotated Int (forced i16): {}", annotated_int);
    println!("Inferred Float (f64 default): {}", inferred_float);
    println!("Annotated Float (forced f32): {}", annotated_float);
    println!("Inferred Char: {}", inferred_char);
    println!("Annotated Char: {}", annotated_char);
}
