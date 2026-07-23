use std::io::{self, Write};

fn main() {
    println!("Welcome to the Interactive Rust Traits & Generics Tutorial!\n");
    wait_for_enter();

    // 1. Generics
    println!("--- 1. Generics (<T>) ---");
    println!("Generics allow us to write code that works with ANY type, instead of rewriting it for each type.");
    println!("Think of <T> as a placeholder for 'Type'.");

    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = find_largest(&number_list);
    println!("The largest number is: {}", largest_number);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = find_largest(&char_list);
    println!("The largest char is: {}", largest_char);

    ask_question(
        "Did we have to write two separate functions (one for i32, one for char)? (yes/no):",
        "no",
        "Correct! We used one generic function `fn find_largest<T>(...)` that handles both."
    );
    wait_for_enter();

    // 2. Traits
    println!("--- 2. Traits (Shared Behavior) ---");
    println!("Traits are like 'interfaces' in other languages.");
    println!("They let us define a behavior (a function) that multiple different types must have.");
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    ask_question(
        "Do both `Tweet` and `NewsArticle` share the same `summarize` function name? (yes/no):",
        "yes",
        "Correct! They both implement the `Summary` trait, so we know they both have a `.summarize()` method."
    );
    wait_for_enter();

    // 3. Trait Bounds
    println!("--- 3. Trait Bounds (T: Trait) ---");
    println!("What if we want a generic function, but we only want types that have a specific behavior?");
    println!("We use Trait Bounds! Like `T: Summary` (T must implement the Summary trait).");

    println!("\nCalling notify on the tweet:");
    notify(&tweet);
    
    println!("\nCalling notify on the article:");
    notify(&article);

    ask_question(
        "Can we pass an `i32` integer to the `notify` function? (yes/no):",
        "no",
        "Correct! The `notify` function demands that the type implements `Summary`, and `i32` does not."
    );
    
    println!("\nTutorial Complete! You've mastered the basics of Traits and Generics! 🦀");
}

// ---------------------------------------------------------
// Helper Functions, Traits & Structs
// ---------------------------------------------------------

// 1. GENERICS
// This function uses Generic type `T`.
// It also has a Trait Bound: `T: PartialOrd + Copy`. 
// It means: T must be comparable (so we can use >) AND it must be copyable (for simplicity here).
fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 2. TRAITS
// We define a trait called Summary. Anything that wants to be a "Summary" 
// MUST have a function called summarize that returns a String.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

// We implement the Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// We implement the Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 3. TRAIT BOUNDS
// This function accepts ANY type (T), AS LONG AS it implements the Summary trait!
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}


// ---------------------------------------------------------
// Interactive prompt functions
// ---------------------------------------------------------
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
