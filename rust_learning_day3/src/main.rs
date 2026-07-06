fn main() {
    // print greeting message
    println!("Hello, Siddhant! Welcome to Day 3: How Rust Works Internally.");

    // testing how rust optimizer works
    // in release mode (cargo build --release), rustc compiles this down to just y = 10 directly
    let x = 10;
    let y = x + 0; 
    println!("Optimized value of y is: {}", y);

    // compile check experiment
    // uncomment these lines to see how rustc catches type errors before running the code
    // let age = "Twenty-three";
    // let result = age + 5; // error: can't add string and integer
    // println!("Result is: {}", result);
}
