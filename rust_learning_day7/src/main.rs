fn main() {
    println!("--- Day 7: Operators in Rust ---\n");

    // 1 & 2. Simple Calculator: Arithmetic operations on two numbers
    let num1 = 25.0;
    let num2 = 4.0;
    
    println!("--- 1 & 2. Simple Calculator ---");
    println!("Numbers: {} and {}", num1, num2);
    println!("Addition: {} + {} = {}", num1, num2, num1 + num2);
    println!("Subtraction: {} - {} = {}", num1, num2, num1 - num2);
    println!("Multiplication: {} * {} = {}", num1, num2, num1 * num2);
    println!("Division: {} / {} = {}", num1, num2, num1 / num2);
    println!("Modulus: {} % {} = {}\n", num1, num2, num1 % num2);

    // 3. Comparison Operators
    let a = 15;
    let b = 20;
    println!("--- 3. Comparison Operators ---");
    println!("Numbers: a = {}, b = {}", a, b);
    println!("a == b : {}", a == b);
    println!("a != b : {}", a != b);
    println!("a > b  : {}", a > b);
    println!("a < b  : {}", a < b);
    println!("a >= b : {}", a >= b);
    println!("a <= b : {}\n", a <= b);

    // 4. Even or Odd Check
    let number = 42;
    println!("--- 4. Even or Odd Check ---");
    if number % 2 == 0 {
        println!("{} is Even\n", number);
    } else {
        println!("{} is Odd\n", number);
    }

    // 5. Login check using && and ||
    let entered_username = "admin";
    let entered_password = "password123";
    let is_admin = true;
    let has_security_token = false;

    println!("--- 5. Login check using Logical Operators ---");
    
    // Correct username AND correct password
    let credentials_correct = entered_username == "admin" && entered_password == "password123";
    
    // Can login if credentials are correct OR if they have a security token
    if credentials_correct || (is_admin && has_security_token) {
        println!("Login Successful! Welcome, {}.\n", entered_username);
    } else {
        println!("Login Failed! Invalid credentials or missing token.\n");
    }

    // 6. Operator Precedence
    println!("--- 6. Operator Precedence ---");
    let result_without_parentheses = 5 + 10 * 2; // 10 * 2 first, then + 5
    let result_with_parentheses = (5 + 10) * 2;  // 5 + 10 first, then * 2

    println!("5 + 10 * 2 = {}", result_without_parentheses);
    println!("(5 + 10) * 2 = {}", result_with_parentheses);
}

