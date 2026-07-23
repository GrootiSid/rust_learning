use std::io::{self, Write};

fn main() {
    println!("Welcome to the Interactive Rust Lifetimes Tutorial!\n");
    wait_for_enter();

    // 1. The Core Idea
    println!("--- 1. The Core Idea of Lifetimes ---");
    println!("A lifetime is Rust's way of ensuring that references are always valid.");
    println!("It prevents 'dangling references' (pointing to memory that has been freed).");
    
    let string1 = String::from("Long-lived string");
    let mut _reference_to_string: &String;

    {
        let _string2 = String::from("Short-lived string");
        // We can safely borrow _string2 here because it's still alive in this scope.
        _reference_to_string = &string1; 
        
        // If we tried to do `_reference_to_string = &_string2;` 
        // the compiler would stop us! Because string2 dies at the end of this block.
        println!("We safely borrowed 'string1' because it outlives this block.");
    }
    
    ask_question(
        "If a variable 'x' is dropped, can you still use a reference to it? (yes/no):",
        "no",
        "Correct! That would be a dangling reference, which Rust prevents."
    );
    wait_for_enter();


    // 2. Lifetime Elision (Automatic Lifetimes)
    println!("--- 2. Lifetime Elision ---");
    println!("Usually, you don't even have to write lifetimes. Rust guesses them for you!");
    println!("This is called 'Lifetime Elision'.");
    
    let word = first_word("Hello world");
    println!("The first word is: {}", word);
    println!("Notice our function 'first_word' takes a reference &str and returns a reference &str.");
    
    ask_question(
        "Did we have to write explicit lifetime annotations like <'a> for the 'first_word' function? (yes/no):",
        "no",
        "Correct! Since there's only one input reference, Rust automatically knows the output reference is tied to it."
    );
    wait_for_enter();


    // 3. Explicit Lifetimes in Functions
    println!("--- 3. Explicit Lifetimes ---");
    println!("When a function takes TWO references, Rust gets confused. It asks: Which one is the output tied to?");
    println!("We use explicit lifetimes like <'a> to tell Rust the relationship.");
    
    let string_a = String::from("short");
    let string_b = String::from("much longer string");
    
    // The longest function uses explicit lifetimes: <'a>
    let longest_str = longest(&string_a, &string_b);
    println!("The longest string is: {}", longest_str);
    
    ask_question(
        "In `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`, does 'a actually change how long the data lives? (yes/no):",
        "no",
        "Correct! Lifetimes are just labels to explain relationships to the compiler. They don't extend the life of data."
    );
    wait_for_enter();


    // 4. Lifetimes in Structs
    println!("--- 4. Lifetimes in Structs ---");
    println!("If a Struct wants to hold a reference, it MUST have a lifetime annotation.");
    println!("This is a promise that the Struct won't outlive the data it's holding.");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    // Our struct holds a reference to a part of the novel.
    let excerpt = Excerpt { part: first_sentence };
    println!("Our struct holds this excerpt: '{}'", excerpt.part);
    
    ask_question(
        "Can the 'excerpt' struct outlive the 'novel' string it borrows from? (yes/no):",
        "no",
        "Correct! The compiler will ensure the struct is dropped before or at the same time as the data it borrows."
    );
    wait_for_enter();


    // 5. The 'static Lifetime
    println!("--- 5. The 'static Lifetime ---");
    println!("'static is a special lifetime that means the reference is valid for the ENTIRE duration of the program.");
    
    let static_string: &'static str = "I am baked directly into the binary!";
    println!("{}", static_string);
    
    ask_question(
        "Are string literals (like \"hello\") automatically given the 'static lifetime? (yes/no):",
        "yes",
        "Correct! Because they are hardcoded into the compiled binary, they live forever."
    );
    
    println!("\nTutorial Complete! You've mastered the basics of Rust Lifetimes! 🦀");
}

// ---------------------------------------------------------
// Helper Functions & Structs
// ---------------------------------------------------------

// Lifetime Elision: Rust knows the returned &str comes from the input &str
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

// Explicit Lifetimes: We must specify <'a> because there are two input references.
// This tells Rust: The returned reference lives as long as the SHORTEST of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with a lifetime. It says: "This struct cannot outlive the reference 'part'."
struct Excerpt<'a> {
    part: &'a str,
}

// Interactive prompt functions
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
