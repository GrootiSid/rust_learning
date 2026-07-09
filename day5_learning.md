# Rust Day 5: Variables, Mutability, Constants & Shadowing 📦

Welcome back! Today, we are writing real Rust code. We'll explore variables, mutability, constants, and a unique feature of Rust called **Shadowing**.

---

## 📦 What is a Variable?

A variable is a named location in memory that stores a value. 
Think of it as labeling a box so you can find what is inside later:

```text
  Label/Name:      [  age  ]
                     │
                     ▼
  Memory Box:     [  22   ] (Value inside the box)
```

In Rust, you create a variable using the `let` keyword:
```rust
let age = 22;
```

---

## 🔒 Mutability (The Marker vs. Whiteboard)

In Rust, **variables are immutable by default**. This means once you write a value, it cannot be changed.

```rust
fn main() {
    let age = 22;
    age = 23; // ❌ COMPILE ERROR: cannot assign twice to immutable variable
}
```

### Why is this the default?
Rust prioritizes safety. If multiple parts of your code have access to a variable and any part can change it without warning, it easily leads to concurrency data races or logic bugs. Making them read-only by default keeps your program predictable.

### ✏️ Making Variables Mutable (`mut`)
If you explicitly want a variable's value to change, use the `mut` keyword:

```rust
fn main() {
    let mut age = 22;
    age = 23; // ✔️ Works perfectly!
}
```

#### The Notebook Analogy:
* **`let` (Immutable)**: Writing with a permanent marker. Once written, it stays.
* **`let mut` (Mutable)**: Writing on a whiteboard. You can erase and replace the value in the exact same spot.

```text
Mutable variable in memory:
  Variable:  [  age  ] ──(overwrites same memory box)──> [  age  ]
  Value:     [  22   ]                                   [  23   ]
```

---

## 💎 Constants (`const`)

Constants are values that are bound to a name and are **never** allowed to change. They must be known at compile-time.

```rust
const PI: f64 = 3.14159;
```

### Difference between `let` and `const`

| Feature | `let` (Immutable Variable) | `const` (Constant) |
| :--- | :--- | :--- |
| **Type Annotation** | Optional (Rust infers it). | **Mandatory** (You must write `: type`). |
| **Mutability** | Can become mutable with `mut`. | **Never** mutable (always fixed). |
| **Value Resolution**| Set at runtime. | Must be set at compile-time. |
| **Naming Convention**| `snake_case` (e.g., `max_users`). | `UPPER_SNAKE_CASE` (e.g., `MAX_USERS`). |
| **Scope** | Block scope. | Can be declared in the global scope. |

---

## 👥 Shadowing

Shadowing occurs when you declare a new variable with the **same name** as a previous variable. The new variable "shadows" (hides) the old one.

```rust
fn main() {
    let x = 5;
    let x = x + 1; // x is now 6
    println!("Value: {}", x);
}
```

### The Student Analogy:
Imagine a student named **Rahul** leaves the classroom, and another student also named **Rahul** enters. When you call out "Rahul", the new Rahul answers. The old Rahul is hidden (shadowed).

### 🔄 Shadowing vs. Mutability
Shadowing does **not** change the value inside the memory box. Instead, it throws away the old box and creates a brand new one with the same name:

```text
Shadowing in memory:
  Variable 1: [  x  ] ──> Value: [  5  ]  (Old box hidden/inaccessible)
  Variable 2: [  x  ] ──> Value: [  6  ]  (New box created on the stack!)
```

### Why use Shadowing?
It allows you to transform a value (e.g., changing its type) without having to create multiple variables with messy names (like `input_str` and `input_int`):

```rust
// Shadowing to change type cleanly
let spaces = "   ";       // String slice type
let spaces = spaces.len(); // Shadowed as an Integer type (3)
```

---

## 💬 Comments

Comments are notes for humans. The compiler skips them completely:
* **Single-line**: Use `//`
* **Multi-line**: Use `/*` to start and `*/` to end.

---

## 📚 Homework: Test Your Knowledge

### 📝 Part 1: Theory Questions
1. What is a variable?
2. Why are variables immutable by default in Rust?
3. What is the difference between `let` and `let mut`?
4. What is the difference between `let` and `const`?
5. What is shadowing, and how does it differ from mutability?
6. Why does Rust encourage immutability?

---

### 💻 Part 2: Practical Exercises
Run the commands inside the Day 5 directory to test the project:

1. **Navigate into the Day 5 folder**:
   ```powershell
   cd rust_learning_day5
   ```
2. **Execute compile validation**:
   ```powershell
   cargo check
   ```
3. **Run your code**:
   ```powershell
   cargo run
   ```
   *Expected Output:*
   ```text
   User Info -> Name: Siddhant, Age: 22, City: Bengaluru
   Starting Salary: 50000
   Updated Salary: 60000
   Constants: App = Siddhant's Rust App, Max Users = 500, Pi = 3.14159
   Number of spaces: 3
   ```
