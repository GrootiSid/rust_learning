use std::io::{self, Write};

fn main() {
    println!("Welcome to the Interactive Rust Ownership Tutorial!\n");
    wait_for_enter();

    // 1. Variable Scope & Dropping
    println!("--- 1. Variable Scope ---");
    println!("In Rust, variables only live inside their 'scope' (usually between {{ }}).");
    {
        let greeting = String::from("Hello there!"); 
        println!("We just created a variable 'greeting' inside a scope.");
        println!("Its value is: {}", greeting);
    } 
    println!("We just left the scope. The variable 'greeting' was automatically dropped by Rust!");
    println!("If we tried to print it now, the compiler would stop us.");
    wait_for_enter();


    // 2. Move Semantics (Transferring Ownership)
    println!("--- 2. Move Semantics ---");
    println!("Let's create a string 's1'.");
    let s1 = String::from("Rust");
    println!("Now we assign s1 to s2: `let s2 = s1;`");
    let s2 = s1; 
    
    println!("Question: Who owns the string now, 's1' or 's2'?");
    ask_question("Type 's1' or 's2':", "s2", "Because ownership MOVED from s1 to s2. s1 is now empty/invalid!");
    println!("s2 owns the string: {}", s2);
    wait_for_enter();


    // 3. Clone (Deep Copy)
    println!("--- 3. Clone ---");
    println!("What if we actually want TWO copies of the data?");
    println!("We use `.clone()`.");
    let original = String::from("Apples");
    let copy = original.clone(); 
    
    println!("Because we cloned, BOTH variables are valid.");
    println!("Original: {}, Copy: {}", original, copy);
    wait_for_enter();


    // 4. Copy Trait (Stack Data)
    println!("--- 4. Copy Trait ---");
    println!("Wait, does everything move?");
    let x = 10;
    let y = x;
    println!("We did `let x = 10; let y = x;`");
    
    ask_question("Did 'x' move to 'y'? (yes/no):", "no", "Correct! Simple types like integers live on the stack and are COPIED automatically.");
    println!("x: {}, y: {}", x, y); 
    wait_for_enter();


    // 5. Ownership and Functions
    println!("--- 5. Functions & Ownership ---");
    println!("Passing a variable to a function works just like assigning it to another variable.");
    let my_string = String::from("Learning");
    takes_ownership(my_string);
    println!("We passed 'my_string' into a function. It was MOVED into the function and is now gone from here.");
    
    let my_int = 42;
    makes_copy(my_int);
    println!("We passed 'my_int' into a function. Since integers are simple, it was COPIED. It's still here: {}", my_int);
    wait_for_enter();


    // 6. Borrowing (References)
    println!("--- 6. Borrowing ---");
    println!("Moving ownership all the time is annoying. Instead, we can BORROW data using references (&).");
    let book = String::from("Rust Programming");
    let length = calculate_length(&book); 
    println!("We passed '&book' to calculate its length.");
    println!("The book '{}' has {} bytes.", book, length); 
    println!("Since we only borrowed it, we still own 'book' and can use it!");
    wait_for_enter();


    // 7. Mutable Borrowing
    println!("--- 7. Mutable Borrowing ---");
    println!("If we want a function to modify our data without taking ownership, we use mutable borrowing (&mut).");
    let mut message = String::from("Hello");
    change_message(&mut message); 
    println!("We passed '&mut message' into a function that added to it.");
    println!("Changed message: {}", message);
    
    println!("\nTutorial Complete! You survived Rust Ownership! 🦀");
}

// Helper Functions

fn wait_for_enter() {
    println!("\n[Press Enter to continue...]");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("--------------------------------------------------");
}

fn ask_question(prompt: &str, expected: &str, explanation: &str) {
    loop {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let answer = input.trim().to_lowercase();
        
        if answer == expected {
            println!("> Correct! {}", explanation);
            break;
        } else {
            println!("> Not quite. Try again! (Hint: Type '{}')", expected);
        }
    }
}

fn takes_ownership(s: String) {
    println!("  [Function] I took ownership of: {}", s);
} 

fn makes_copy(i: i32) {
    println!("  [Function] I made a copy of: {}", i);
} 

fn calculate_length(s: &String) -> usize {
    s.len() 
} 

fn change_message(s: &mut String) {
    s.push_str(", world!"); 
}
