# Rust Day 6: Data Types (The Foundation of Memory) 🦀

Welcome back, bhai! ❤️ Today, we are diving deep into **Rust Data Types**. 

If you master this topic, understanding how Rust manages memory under the hood (Ownership, Borrowing, Structs, and Enums) will become a breeze. Let's make memory management fun and simple!

---

## 📦 What is a Data Type?

Suppose I give you a box and say: **"Isme kuch store karo."** (Store something in this).

Your first question will be: **"Kya store karna hai?"** (What do I need to store?)

This is because you cannot put a giant elephant inside a small pencil box! 😂 

In computer programming, the same rule applies. Before saving any data, the computer needs to know:
1. **How much memory to reserve?** (Size of the box)
2. **What kind of data will be stored?** (Type of content)
3. **How should it process that data?** (How to read it)

> **Definition:** A **data type** defines what kind of value a variable can store and how much memory it needs.

### Example:
```rust
let age = 22;
```
Here, the Rust compiler looks at `22` and automatically says: *"Aha! This is a whole number (integer). I will assign it an integer type."*

---

## 💧 Why Data Types Matter? (The Water Bottle Analogy)

Imagine you have a **1-liter** water bottle:

```text
   ┌──────┐
   │      │ ◄─── 1 Liter Capacity
   │      │
   │      │
   └──────┘
```

What happens if you try to pour **5 liters** of water into it?
* **Result:** ❌ It overflows, leaks, and messes up everything.

In programming, this overflow is a major bug (called **Integer Overflow** or memory corruption). Every data type has a fixed memory size. If you try to store a value larger than what the reserved memory can hold, your program will crash or behave unpredictably. Rust protects you from this by having strict data type rules!

---

## 🗺️ Rust Data Types Hierarchy

Rust categorizes its data types into two main groups:

```text
                     ┌────────────────────────┐
                     │    Rust Data Types     │
                     └───────────┬────────────┘
                                 │
         ┌───────────────────────┴───────────────────────┐
         ▼                                               ▼
 ┌──────────────┐                                ┌───────────────┐
 │ Scalar Types │ (Single Value)                 │Compound Types │ (Group of Values)
 └──────┬───────┘                                └───────┬───────┘
        ├─ Integer (e.g., 22, -100)                      ├─ Tuple (Different types: ("Sid", 22))
        ├─ Float   (e.g., 3.14, -0.5)                    └─ Array (Same type, fixed size: [1, 2, 3])
        ├─ Boolean (e.g., true, false)
        └─ Char    (e.g., 'A', '🦀')
```

---

## 1. Scalar Types (Single Values)

A **scalar** type represents a single, unique value. Rust has four primary scalar types:

### A. Integers (Whole Numbers)
Integers are numbers without decimals. They can be positive, negative, or zero.

Rust splits integers into **Signed** (can be positive or negative) and **Unsigned** (only positive).

| Size | Signed (with `i` - Integer) | Unsigned (with `u` - Unsigned) |
| :--- | :--- | :--- |
| **8-bit** | `i8` | `u8` |
| **16-bit** | `i16` | `u16` |
| **32-bit** | `i32` *(Default)* | `u32` |
| **64-bit** | `i64` | `u64` |
| **128-bit**| `i128` | `u128` |
| **Arch** | `isize` | `usize` |

#### 💡 Signed vs. Unsigned:
* **Signed (`i`)**: Can store both positive and negative values (uses 1 bit for the sign: `+` or `-`).
* **Unsigned (`u`)**: Can **only** store positive values (or zero), which gives them double the positive range since they don't waste a bit on the sign.

#### 🧮 Understanding Bits & Memory Ranges:
Memory is measured in bits. **8 bits = 1 byte**.
* For a **`u8`** (8-bit unsigned integer): 
  * Total values it can hold = $2^8 = 256$ values.
  * Range: `0` to `255`.
* For an **`i8`** (8-bit signed integer):
  * Range: $-(2^7)$ to $2^7 - 1$ = `-128` to `127`.

> **Rule of Thumb:**
> * If you don't specify a type, Rust defaults to **`i32`**. It is highly optimized for modern CPUs.
> * Use **`usize` / `isize`** when indexing collections (like arrays or vectors) because their size depends on your computer's CPU architecture (64-bit on a 64-bit machine, 32-bit on a 32-bit machine).

---

### B. Floating-Point Types (Decimals)
Floating-point types represent numbers with decimal points (fractional numbers).

Rust has two floating-point types:
1. **`f32`**: Single-precision float (occupies 32 bits / 4 bytes).
2. **`f64`**: Double-precision float (occupies 64 bits / 8 bytes). **(Default in Rust)**

```rust
let x = 2.0;      // Defaults to f64 (more precise)
let y: f32 = 3.0; // Explicitly declared f32
```

> **Why default to `f64`?** 
> Modern CPUs process `f64` just as fast as `f32`, but `f64` offers much higher precision (fewer rounding errors).

---

### C. Boolean Type
Booleans are the simplest type. They can only hold one of two values: **`true`** or **`false`**. They occupy **1 byte** of memory.

```rust
let is_rust_awesome: bool = true;
let is_it_raining = false; // inferred
```

---

### D. Character Type (`char`)
The `char` type represents a single Unicode Scalar Value. 
* **Crucial Rule:** Characters must be wrapped in **single quotes** (`'A'`). Double quotes (`"A"`) denote a String slice.

```rust
let letter: char = 'z';
let heart_emoji: char = '❤'; // Yes, Emojis are valid Unicode characters!
let hindi_char: char = 'अ';  // Works perfectly!
```
*Unlike other languages where a character is 1 byte, Rust's `char` is **4 bytes** in size, allowing it to represent any Unicode character in the world (alphabets, symbols, emojis, etc.).*

---

## 2. Compound Types (Grouped Values)

Compound types group multiple values into a single type. Rust has two primitive compound types:

### A. Tuples
A tuple is a general way of grouping together a **variety of types** into one compound type. Tuples have a **fixed length**; once declared, they cannot grow or shrink.

```rust
// A tuple containing a string, an integer, and a float
let student: (&str, u8, f64) = ("Siddhant", 22, 9.5);
```

#### 🔍 Accessing Tuple Elements:
We access elements in a tuple using **dot notation** followed by the index (which starts at 0):

```rust
let name = student.0;
let age = student.1;
let cgpa = student.2;

println!("Name is {}, Age is {}, CGPA is {}", name, age, cgpa);
```

---

### B. Arrays
An array is a collection of multiple values of the **same type** stored in a contiguous block of memory. Like tuples, arrays have a **fixed length**.

```rust
// An array containing 5 marks (all must be the same type)
let marks: [u32; 5] = [95, 90, 85, 99, 80];
```

#### 🔍 Accessing Array Elements:
We access array elements using **square brackets** and index numbers:

```rust
let first_mark = marks[0];
let last_mark = marks[4];
```

| Type | Data Types allowed | Memory Layout | Access Syntax |
| :--- | :--- | :--- | :--- |
| **Tuple** | Mixed Types (e.g., `&str`, `i32`, `bool`) | Stack | `.0`, `.1`, `.2` |
| **Array** | Same Type (e.g., all `i32`) | Stack (Contiguous) | `[0]`, `[1]`, `[2]` |

---

## 🧠 Type Inference vs. Type Annotation

### 1. Type Inference
Rust is smart. It figures out what type your variable should be based on the value you assign to it:
```rust
let score = 95; // Rust infers this as i32
```

### 2. Type Annotation
Sometimes, we want to explicitly dictate the exact type. We do this by adding a colon `:` and the type name:
```rust
let score: u16 = 95; // We explicitly force score to be a u16 (16-bit unsigned integer)
```

---

## 📊 Summary Table

| Type | Example | Size in Memory | Purpose |
| :--- | :--- | :--- | :--- |
| **`i32`** | `-10` | 4 Bytes (32 bits) | General-purpose signed whole numbers |
| **`u32`** | `25` | 4 Bytes (32 bits) | General-purpose positive-only whole numbers |
| **`f64`** | `3.14` | 8 Bytes (64 bits) | Highly precise decimal calculations |
| **`bool`** | `true` | 1 Byte (8 bits) | Storing binary flags / conditional checks |
| **`char`** | `'A'`, `'🦀'` | 4 Bytes (32 bits) | Storing a single Unicode character |
| **`tuple`** | `("John", 22)` | Varies | Grouping different types of data |
| **`array`** | `[1, 2, 3]` | Varies | Grouping fixed number of same-type data |

---

## ❌ Common Beginner Mistakes

### 1. Single vs. Double Quotes for Chars
```rust
let my_char = "A"; // ❌ Error! Double quotes mean String slice (&str)
let my_char = 'A'; // ✔️ Correct! Single quotes represent char
```

### 2. Unsigned Types holding Negative Values
```rust
let temperature: u32 = -5; // ❌ Compile Error! Unsigned integers cannot be negative
```

### 3. Mixing Types in Arrays
```rust
let my_array = [10, "hello"]; // ❌ Compile Error! Arrays can only store one type
```

---

## 🏆 Best Practices

1. **Let Rust Infer Types**: Don't clutter your code with explicit type annotations when they are obvious. Let the compiler do its job!
2. **Use `i32` by Default**: For normal integers, stick to `i32`.
3. **Use `u32` or `usize` for Counting/Index**: Use `usize` when you deal with lengths, sizes, or array indices.
4. **Choose `f64` over `f32`**: For decimals, defaults are best unless you are memory-constrained (e.g., embedded systems).

---

## 📚 Homework: Solutions

### 📝 Part 1: Theory Questions

#### 1. What is a data type?
A data type is a classification of data that tells the compiler how much memory to reserve, what kind of value is stored, and how it should be processed.

#### 2. What is the difference between scalar and compound data types?
* **Scalar types** represent a single, singular value (like a single number `45`, character `'C'`, or bool `true`).
* **Compound types** group multiple values together (like a tuple grouping `("Sid", 22, 9.5)` or an array grouping `[90, 80, 70]`).

#### 3. What is the difference between signed and unsigned integers?
* **Signed (`i`)** integers can represent both positive and negative numbers (e.g., `-10`, `0`, `50`).
* **Unsigned (`u`)** integers can only store positive numbers or zero (e.g., `0`, `150`). They cannot store negative numbers.

#### 4. Why does Rust have multiple integer types?
To allow developers to optimize memory usage. If a number is small (like age: `0-120`), using `u8` saves memory compared to using `i128`, which is essential for embedded systems or high-performance apps.

#### 5. What is type inference?
Type inference is the compiler's ability to automatically deduce the data type of a variable at compile time based on the value assigned to it, without the programmer writing it explicitly.

#### 6. What is type annotation?
Type annotation is when a programmer explicitly specifies the data type of a variable during declaration (e.g., `let x: u64 = 10;`).

#### 7. What is the difference between a tuple and an array?
* A **tuple** can group values of *different* data types (e.g., `(&str, i32, f64)`).
* An **array** can only group values of the *same* data type (e.g., all `i32`). Both have fixed lengths in Rust.

#### 8. Why does `char` use single quotes while strings use double quotes?
In Rust, `char` represents a single Unicode character and is represented in memory as a 4-byte scalar value, defined using single quotes (`'A'`). A string is a sequence of characters and is represented differently in memory, defined using double quotes (`"A"`).

---

### 💻 Part 2: Practical Exercises (Code & Validation)

The codebase is located in: [rust_learning_day6/src/main.rs](file:///d:/rust_learning/rust_learning_day6/src/main.rs)

#### Run commands to compile and run:
```powershell
cd rust_learning_day6
cargo run
```

#### Expected Output:
```text
--- Question 1: Storing Personal Info ---
Name: Siddhant
Age: 22 years
Height: 5.9 feet
Logged In: true

--- Question 2: Scalar Types Demonstration ---
i32 (Signed): -42
u32 (Unsigned): 100
f64 (Float): 3.14159
bool (Boolean): false
char (Character): 🦀

--- Question 3: Tuple containing Name, Age, and CGPA ---
Tuple Name: Siddhant
Tuple Age: 22
Tuple CGPA: 9.5

--- Question 4: Array of Five Marks ---
Marks Array: [95, 90, 85, 99, 80]
First Element (Index 0): 95
Last Element (Index 4): 80

--- Question 5: Type Annotation vs. Type Inference ---
Inferred Int (i32 default): 500
Annotated Int (forced i16): 500
Inferred Float (f64 default): 99.9
Annotated Float (forced f32): 99.9
Inferred Char: A
Annotated Char: A
```
