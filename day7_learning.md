# Day 7 – Operators in Rust (How Programs Make Decisions)

Welcome back, bhai! ❤️

Aaj hum ek aise topic par aa gaye hain jo har programming language ka backbone hai.

Agar variables data store karte hain...

To operators us data par kaam karte hain.

Without operators, tum calculator, login system, banking app, game, ya backend API kuch bhi nahi bana sakte.

🤔 First Question...
What is an Operator?

Real life example.

Suppose I ask you:

`5 + 3`

Tum turant bolte ho:

`8`

Yaha:

`+`

kya hai?

Ye ek Operator hai.

Aur

`5` aur `3`

kya hain?

Ye Operands hain.

Formula:

`Operand Operator Operand`

`5      +        3`

Why Do We Need Operators?

Suppose you're making an e-commerce website.

User buys:

Laptop = ₹50,000

GST = 18%

Discount = 10%

Shipping = ₹500

Total calculate karna padega.

Without operators impossible.

## Rust Operators

Rust operators are divided into several categories.

Operators

│

├── Arithmetic

├── Assignment

├── Comparison

├── Logical

├── Bitwise

├── Range

└── Others

Aaj hum sabse important operators seekhenge.

### 1. Arithmetic Operators

Ye mathematical calculations ke liye use hote hain.

**Addition (+)**
```rust
fn main() {
    let a = 10;
    let b = 20;
    let sum = a + b;
    println!("{}", sum);
}
```
Output:
`30`

**Subtraction (-)**
```rust
let result = 20 - 5;
```
Output:
`15`

**Multiplication (*)**
```rust
let result = 5 * 4;
```
Output:
`20`

**Division (/)**
```rust
let result = 20 / 5;
```
Output:
`4`

⚠️ **Important**

Look carefully.

```rust
let result = 5 / 2;
println!("{}", result);
```

What will be the output?

Many beginners say `2.5`.

Wrong.

Output: `2`

Why?

Because both values are integers.

Rust removes the decimal part.

This is called Integer Division.

If you want `2.5`:

Then use floating point.

```rust
let result = 5.0 / 2.0;
println!("{}", result);
```

Output: `2.5`

**Modulus (%)**

Returns the remainder.

```rust
let result = 10 % 3;
```
Output: `1`

Because `10 ÷ 3`

Quotient = 3
Remainder = 1

**Real Use Case**

How to check if a number is even?

`if number % 2 == 0`

Then Even
Otherwise Odd

**Arithmetic Summary**
| Operator | Meaning |
|---|---|
| + | Addition |
| - | Subtraction |
| * | Multiplication |
| / | Division |
| % | Remainder |

### 2. Assignment Operators

Basic assignment.

```rust
let age = 22;
```
Store `22` inside `age`.

`+=`

Instead of `x = x + 5;`

Write `x += 5;`

Example:

```rust
let mut score = 100;
score += 20;
println!("{}", score);
```

Output: `120`

`-=`
`score -= 10;`

`*=`
`score *= 2;`

`/=`
`score /= 5;`

`%=`
`score %= 3;`

**Assignment Summary**
| Operator | Example |
|---|---|
| = | x = 5 |
| += | x += 2 |
| -= | x -= 2 |
| *= | x *= 2 |
| /= | x /= 2 |
| %= | x %= 2 |

### 3. Comparison Operators

Comparison operators always return: `true` or `false`

**Equal**
`10 == 10` (Output: `true`)

**Not Equal**
`10 != 20` (Output: `true`)

**Greater Than**
`20 > 15` (Output: `true`)

**Less Than**
`10 < 5` (Output: `false`)

**Greater Than Equal**
`10 >= 10` (Output: `true`)

**Less Than Equal**
`10 <= 20` (Output: `true`)

**Comparison Summary**
| Operator | Meaning |
|---|---|
| == | Equal |
| != | Not Equal |
| > | Greater |
| < | Less |
| >= | Greater or Equal |
| <= | Less or Equal |

**Real Example**

Login system.

```rust
let password = "12345";
```
User enters `12345`.
Check `entered == password`.
Result `true`.
User logged in.

### 4. Logical Operators

Suppose a website says You must:
✔ Age > 18
AND
✔ Have Driving License

Both conditions must be true.
This is where logical operators help.

**AND (&&)**

Truth Table
| A | B | Result |
|---|---|---|
| true | true | true |
| true | false | false |
| false | true | false |
| false | false | false |

Example:
```rust
let age = 20;
let has_license = true;
println!("{}", age >= 18 && has_license);
```
Output: `true`

**OR (||)**

Only one condition needs to be true.

Truth Table
| A | B | Result |
|---|---|---|
| true | true | true |
| true | false | true |
| false | true | true |
| false | false | false |

Example:
Weekend OR Holiday
You don't have to work.

**NOT (!)**

Reverses a boolean.

`true` becomes `false`.

Example:
```rust
let is_logged_in = true;
println!("{}", !is_logged_in);
```
Output: `false`

**Logical Summary**
| Operator | Meaning |
|---|---|
| && | AND |
| \|\| | OR |
| ! | NOT |

### 5. Bitwise Operators (Introduction)

These work directly on binary values.

Operators: `&`, `|`, `^`, `<<`, `>>`

Example:
5 = 0101
3 = 0011

These are mostly used in:
- Embedded systems
- Operating systems
- Networking
- Device drivers
- Performance optimization

Don't worry—we'll study them in detail later.

### Operator Precedence

Suppose:
```rust
let result = 2 + 3 * 4;
```
Output? Many beginners say 20.
Actually 14.
Because multiplication happens first.
Like mathematics.

`2 + (3 × 4)` -> `2 + 12` -> `14`

Use parentheses to make your intention clear.
```rust
let result = (2 + 3) * 4;
```
Output: `20`

**Real World Example**

Imagine an ATM.
Conditions: PIN Correct AND Balance > Withdraw Amount
Rust: `pin_correct && balance >= amount`
Only then Cash Dispensed.

### Common Beginner Mistakes

❌ Using `=` instead of `==`
Wrong: `if age = 18` (`=` assigns a value)
Correct: `if age == 18`

❌ Integer Division
`5 / 2`
Output: `2`, Not `2.5`

❌ Forgetting mut
```rust
let x = 5;
x += 2;
```
Error.
Need: `let mut x = 5;`

### Best Practices

✅ Use parentheses when expressions become complex.
✅ Use meaningful variable names.
✅ Prefer `==` for comparison and `=` only for assignment.
✅ Remember integer division truncates the decimal part.

## Summary

Today you learned:
✅ Arithmetic Operators
✅ Assignment Operators
✅ Comparison Operators
✅ Logical Operators
✅ Introduction to Bitwise Operators
✅ Operator Precedence

## Homework

### Theory Answers
1. **What is an operator?**
   An operator is a symbol (like `+`, `-`, `==`) that performs an operation on one or more operands (values or variables) to produce a result.
2. **What is the difference between `=` and `==`?**
   `=` is an assignment operator used to store a value in a variable. `==` is a comparison operator used to check if two values are equal.
3. **Why does `5 / 2` produce `2`?**
   Because both `5` and `2` are integers. Rust performs integer division, which truncates (removes) the decimal part.
4. **What does the modulus operator (`%`) do?**
   It returns the remainder of a division operation. E.g., `10 % 3` is `1`.
5. **What is the difference between `&&` and `||`?**
   `&&` (AND) requires all conditions to be true for the result to be true. `||` (OR) requires only one condition to be true for the result to be true.
6. **Why is operator precedence important?**
   It determines the order in which operations are evaluated in an expression without parentheses (e.g., multiplication before addition), ensuring predictable and correct results.

### Coding Practice (See rust_learning_day7 project)

Write programs to:
1. Perform addition, subtraction, multiplication, division, and modulus on two numbers.
2. Create a simple calculator that prints all arithmetic results.
3. Compare two numbers using all comparison operators.
4. Check whether a number is even or odd using `%`.
5. Simulate a login check using `&&` and `||`.
6. Experiment with parentheses to see how operator precedence affects the result.
