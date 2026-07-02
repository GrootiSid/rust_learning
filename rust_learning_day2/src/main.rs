fn main() {
    // 1. Data Types
    let name = "Siddhant";      // String slice
    let age = 23;               // Integer
    let gpa = 8.5;              // Float
    let is_learning = true;     // Boolean
    let grade = 'A';            // Character
    
    // Tuple (groups different types together)
    let info = ("Siddhant", 23, 8.5);
    
    // Array (groups same types together)
    let numbers = [1, 2, 3];

    println!("Name: {}, Age: {}, GPA: {}, Learning: {}", name, age, gpa, is_learning);
    println!("Grade: {}, Tuple age: {}, Array first element: {}", grade, info.1, numbers[0]);


    // 2. Variable Shadowing
    let x = 5;
    let x = x + 1; // Shadows the previous 'x' (now 6)
    {
        let x = x * 2; // Shadows 'x' to 12 only inside this inner scope
        println!("Inner scope x: {}", x);
    }
    println!("Outer scope x: {}", x); // x is still 6 here


    // 3. Control Flow (if / else)
    let number = 10;
    if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is 5 or less");
    }

    // You can assign the result of an 'if' expression directly to a variable
    let status = if age >= 18 { "Adult" } else { "Minor" };
    println!("User is an: {}", status);


    // 4. Loops

    // While loop
    let mut countdown = 3;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }

    // For loop over an array
    let fruits = ["Apple", "Banana", "Cherry"];
    for fruit in fruits {
        println!("I like {}", fruit);
    }

    // For loop over a range (1 to 3 inclusive)
    for num in 1..=3 {
        println!("Range number: {}", num);
    }

    // Infinite loop that stops with break and returns a value
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 10; // stops the loop and returns 50
        }
    };
    println!("Loop returned: {}", result);
}
