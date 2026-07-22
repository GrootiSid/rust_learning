# 🦀 Rust Ownership — Complete Guide (Basic → Advanced)

> **Goal:** Understand ownership so well you can *recall* it instantly, not just memorize it.
> Every section has: **Concept → Code → Why it matters**.

---

## 📌 Table of Contents

1. [Why Ownership Exists](#1-why-ownership-exists)
2. [The 3 Rules of Ownership](#2-the-3-rules-of-ownership)
3. [Stack vs Heap (the foundation)](#3-stack-vs-heap-the-foundation)
4. [Move Semantics](#4-move-semantics)
5. [Clone vs Copy](#5-clone-vs-copy)
6. [Ownership & Functions](#6-ownership--functions)
7. [References & Borrowing](#7-references--borrowing)
8. [Borrowing Rules (The Big 2)](#8-borrowing-rules-the-big-2)
9. [Slices](#9-slices)
10. [Ownership in Structs & Enums](#10-ownership-in-structs--enums)
11. [Lifetimes](#11-lifetimes)
12. [Smart Pointers (Box, Rc, RefCell, Arc)](#12-smart-pointers-box-rc-refcell-arc)
13. [Real Project: Task Manager (CLI)](#13-real-project-task-manager-cli)
14. [Scenario-Based Q&A](#14-scenario-based-qa)
15. [Quick Recall Cheat Sheet](#15-quick-recall-cheat-sheet)

---

## 1. Why Ownership Exists

Most languages manage memory in one of two ways:

| Approach | Example Languages | Problem |
|---|---|---|
| Manual memory management | C, C++ | Easy to forget `free()` → memory leaks, dangling pointers, double free |
| Garbage Collector (GC) | Java, Python, Go | Safe, but adds runtime overhead & unpredictable pauses |

**Rust's answer:** Ownership — a set of rules checked **at compile time**, with **zero runtime cost**.

> 💡 **Recall trick:** "Rust gives you C's speed with a built-in safety inspector that works before your program even runs."

---

## 2. The 3 Rules of Ownership

```
1. Each value in Rust has a variable that's its OWNER.
2. There can only be ONE owner at a time.
3. When the owner goes out of scope, the value is DROPPED (freed).
```

```rust
fn main() {
    {
        let s = String::from("hello"); // s is the owner
        println!("{}", s);
    } // <- scope ends, s is dropped, memory freed automatically
}
```

> 💡 **Recall trick:** Think of ownership like a **house deed**. Only one person holds the deed at a time. When that person "moves out" (scope ends), the house is demolished (memory freed).

---

## 3. Stack vs Heap (the foundation)

Ownership rules matter most for **heap** data because heap data doesn't have a fixed, known size at compile time.

| | Stack | Heap |
|---|---|---|
| Speed | Fast | Slower |
| Size | Must be known at compile time | Can grow/shrink at runtime |
| Example types | `i32`, `bool`, `char`, fixed arrays | `String`, `Vec<T>`, `Box<T>` |
| Access | LIFO (push/pop) | Via pointer |

```rust
let x = 5;              // stack — simple, fixed size
let s = String::from("hi"); // heap — data lives on heap, pointer lives on stack
```

> 💡 **Recall trick:** Stack = a stack of plates (quick, fixed size). Heap = a warehouse (flexible size, needs an address/pointer to find things).

---

## 4. Move Semantics

When you assign a heap-allocated value to another variable, Rust **moves** ownership — it does **not** deep copy.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED into s2

    // println!("{}", s1); // ❌ ERROR: value borrowed after move
    println!("{}", s2); // ✅ works
}
```

**Why?** If both `s1` and `s2` pointed to the same heap memory and both went out of scope, Rust would try to free the same memory twice → **double free error**. So Rust invalidates `s1` the moment `s2` takes over.

```
s1 ──┐
     ├──> [ "hello" on heap ]   (before move)
s2 ──┘

After `let s2 = s1;`:

s1  ✗ (invalidated)
s2 ──────> [ "hello" on heap ]
```

> 💡 **Recall trick:** "Move = handing over your ONLY house key. You can't open the door anymore."

### Move also happens with function calls

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
} // s dropped here

fn main() {
    let s = String::from("world");
    takes_ownership(s);
    // println!("{}", s); // ❌ ERROR — s was moved into the function
}
```

---

## 5. Clone vs Copy

### `.clone()` — explicit deep copy (heap data)

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copies the heap data

println!("{} {}", s1, s2); // ✅ both valid
```

### `Copy` trait — implicit copy (stack-only, simple data)

Types like `i32`, `f64`, `bool`, `char`, and tuples of these implement `Copy`. They're copied automatically instead of moved.

```rust
let x = 5;
let y = x; // COPIED, not moved
println!("{} {}", x, y); // ✅ both valid — no error!
```

| Rule | Types |
|---|---|
| `Copy` (cheap, stack only) | integers, floats, `bool`, `char`, tuples of Copy types |
| `Clone` (explicit, can be expensive) | `String`, `Vec<T>`, `HashMap`, custom structs (via `#[derive(Clone)]`) |

> 💡 **Recall trick:** "If it fits entirely on the stack and copying is cheap → Rust does it silently (`Copy`). If it touches the heap → you must ask for it explicitly (`.clone()`)."

---

## 6. Ownership & Functions

Passing a value to a function **moves** it (unless it's `Copy`), and returning a value **moves ownership out**.

```rust
fn gives_ownership() -> String {
    let s = String::from("gift");
    s // ownership moved OUT to the caller
}

fn takes_and_gives_back(s: String) -> String {
    s // takes ownership, then gives it back
}

fn main() {
    let s1 = gives_ownership();          // s1 owns the String
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);   // s2 moved in, s3 owns it now
}
```

**Problem:** Constantly passing ownership back and forth is tedious. Solution → **References (Borrowing)**. That's next.

---

## 7. References & Borrowing

Instead of transferring ownership, you can **borrow** a value using `&`.

```rust
fn calculate_length(s: &String) -> usize { // borrows s, doesn't own it
    s.len()
} // s goes out of scope, but nothing is dropped (it doesn't own the data)

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass a reference
    println!("The length of '{}' is {}.", s1, len); // ✅ s1 still valid!
}
```

> 💡 **Recall trick:** "Borrowing = lending your book to a friend. You still own it; they just read it and give it back."

### Mutable References

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // "hello, world"
}
```

---

## 8. Borrowing Rules (The Big 2)

```
Rule 1: You can have EITHER
        - any number of immutable references (&T), OR
        - exactly ONE mutable reference (&mut T)
        ... but not both at the same time.

Rule 2: References must always be VALID (no dangling references).
```

```rust
let mut s = String::from("hello");

let r1 = &s; // ✅ immutable borrow
let r2 = &s; // ✅ another immutable borrow — fine
println!("{} {}", r1, r2);
// r1, r2 no longer used after this point (NLL - Non-Lexical Lifetimes)

let r3 = &mut s; // ✅ OK now, because r1/r2 are "done"
println!("{}", r3);
```

```rust
// ❌ This WON'T compile:
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s; // ERROR: cannot borrow as mutable because it's already borrowed as immutable
println!("{}, {}", r1, r2);
```

**Why this rule exists:** It prevents **data races** at compile time — no thread/reference can unexpectedly mutate data while another is reading it.

### No Dangling References

```rust
fn dangle() -> &String {   // ❌ ERROR
    let s = String::from("hello");
    &s
} // s dropped here, but we tried to return a reference to it!
```

Fix: return the owned `String` itself, not a reference.

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership moved out — totally fine
}
```

> 💡 **Recall trick:** "Many readers OR one writer, never both. And never hand out a reference to something that's about to disappear."

---

## 9. Slices

A **slice** is a reference to a *contiguous part* of a collection. It doesn't own data either.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // string slice
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s); // word = "hello"
    println!("{}", word);
}
```

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3]; // [2, 3] — borrows part of the array
```

> 💡 **Recall trick:** "A slice is a *view* into data — a window, not a copy."

---

## 10. Ownership in Structs & Enums

```rust
struct User {
    username: String, // struct OWNS this String
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        active: true,
    };

    let username_copy = user1.username; // MOVES username out of user1
    // println!("{}", user1.username); // ❌ ERROR: partial move
    println!("{}", username_copy); // ✅ ok
}
```

**Best practice:** Structs generally should own their data (`String` not `&str`) unless you use lifetimes (next section) to explicitly manage borrowed data.

```rust
// Struct holding a REFERENCE needs a lifetime annotation:
struct UserRef<'a> {
    username: &'a str,
}
```

---

## 11. Lifetimes

Lifetimes ensure references are **valid for as long as they're used** — preventing dangling references, especially across function boundaries.

### The Problem

```rust
fn longest(x: &str, y: &str) -> &str { // ❌ ERROR: missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
```

Rust can't tell whether the returned reference lives as long as `x` or `y`. So we annotate:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let result = longest(s1.as_str(), s2.as_str());
    println!("Longest: {}", result);
}
```

**`'a` doesn't change how long anything lives** — it just tells the compiler: *"the returned reference will be valid for the smaller of x's and y's lifetimes."*

> 💡 **Recall trick:** "Lifetimes are labels, not timers. They describe relationships between references, they don't extend anyone's life."

### Lifetime Elision (when you DON'T need to write `'a`)

Rust has 3 rules that let it infer lifetimes automatically in common cases:

```rust
fn first_word(s: &str) -> &str { // no 'a needed — elision rules apply
    ...
}
```

### Struct Lifetimes

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence };
    println!("{}", excerpt.part);
}
```

### The `'static` Lifetime

```rust
let s: &'static str = "I live for the entire program"; // string literals are 'static
```

> 💡 **Recall trick:** `'static` = "lives as long as the program itself" (like string literals baked into the binary).

---

## 12. Smart Pointers (Box, Rc, RefCell, Arc)

Sometimes ownership rules are too strict for a use case. Smart pointers relax specific rules, safely.

### `Box<T>` — Heap allocation, single owner

```rust
let b = Box::new(5); // 5 is stored on the heap
println!("{}", b);
```
**Use case:** Recursive types (size unknown at compile time).

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### `Rc<T>` — Reference Counted, MULTIPLE owners (single-threaded)

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a); // increments ref count, NOT a deep copy
    let c = Rc::clone(&a);

    println!("Count: {}", Rc::strong_count(&a)); // 3
} // as each goes out of scope, count decreases; freed when count = 0
```

> 💡 **Recall trick:** "`Rc` = a shared Netflix subscription. Everyone has a key, and the account stays alive as long as someone's still watching."

### `RefCell<T>` — Interior mutability (borrow rules checked at RUNTIME)

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    *data.borrow_mut() += 1; // mutate even though `data` isn't declared `mut`
    println!("{}", data.borrow()); // 6
}
```

**Rc + RefCell combo** — the classic pattern for shared, mutable data in single-threaded code:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
let clone1 = Rc::clone(&shared);
clone1.borrow_mut().push(4);
println!("{:?}", shared.borrow()); // [1, 2, 3, 4]
```

### `Arc<T>` — Atomic Rc, for MULTI-THREADED sharing

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            println!("Thread {}: {:?}", i, data);
        }));
    }
    for h in handles { h.join().unwrap(); }
}
```

| Type | Owners | Mutability | Thread-safe? |
|---|---|---|---|
| `Box<T>` | 1 | via `&mut` | N/A (single owner) |
| `Rc<T>` | Many | immutable | ❌ No |
| `Rc<RefCell<T>>` | Many | mutable (runtime-checked) | ❌ No |
| `Arc<T>` | Many | immutable | ✅ Yes |
| `Arc<Mutex<T>>` | Many | mutable (runtime-checked) | ✅ Yes |

> 💡 **Recall trick:** "Single-threaded sharing → `Rc`. Multi-threaded sharing → `Arc`. Need mutation too → wrap in `RefCell` (single-thread) or `Mutex` (multi-thread)."

---

## 13. Real Project: Task Manager (CLI)

A small but realistic project demonstrating **every ownership concept above** together.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,   // owned data
    done: bool,
}

impl Task {
    fn new(id: u32, title: &str) -> Self {
        // `title: &str` is BORROWED, then `.to_string()` creates an OWNED String
        Task { id, title: title.to_string(), done: false }
    }
}

// TaskManager OWNS the list of tasks.
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    // Takes ownership of `task` — the manager becomes the sole owner.
    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // Borrows self immutably — read-only access, many callers can do this concurrently.
    fn list_tasks(&self) {
        for task in &self.tasks { // borrowing each Task, not moving it
            println!("[{}] {} - done: {}", task.id, task.title, task.done);
        }
    }

    // Borrows self mutably — exclusive access needed to mutate.
    fn complete_task(&mut self, id: u32) -> Option<&Task> {
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.done = true;
                return Some(task); // returns a reference tied to self's lifetime
            }
        }
        None
    }

    // Returns a slice — a borrowed VIEW into the Vec, no copying.
    fn pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| !t.done).collect()
    }
}

fn main() {
    let mut manager = TaskManager::new();

    manager.add_task(Task::new(1, "Learn ownership"));
    manager.add_task(Task::new(2, "Learn borrowing"));
    manager.add_task(Task::new(3, "Build a project"));

    println!("--- All tasks ---");
    manager.list_tasks();

    manager.complete_task(1);

    println!("--- Pending tasks ---");
    for t in manager.pending_tasks() {
        println!("{} (id {})", t.title, t.id); // just borrowing, never owning
    }

    // --- Shared, mutable state across "threads" (simulated with Rc<RefCell<>>) ---
    let shared_manager = Rc::new(RefCell::new(manager));

    let handle1 = Rc::clone(&shared_manager);
    handle1.borrow_mut().add_task(Task::new(4, "Shared task from handle1"));

    let handle2 = Rc::clone(&shared_manager);
    println!("--- After shared mutation ---");
    handle2.borrow().list_tasks();

    println!("Total owners of manager: {}", Rc::strong_count(&shared_manager));
}
```

### What this project teaches, line by line:

| Concept | Where it appears |
|---|---|
| Ownership transfer | `add_task(task)` — the `Task` moves into the `Vec` |
| Borrowing (`&self`) | `list_tasks`, `pending_tasks` — read-only access |
| Mutable borrow (`&mut self`) | `complete_task`, `add_task` — exclusive write access |
| Slices/iterators as views | `pending_tasks` returns `Vec<&Task>` — references, no copies |
| `Clone` derive | `#[derive(Clone)]` on `Task` for convenience |
| `Rc<RefCell<T>>` | Shared, mutable `TaskManager` across multiple "owners" |
| Reference counting | `Rc::strong_count` shows live owner count |

---

## 14. Scenario-Based Q&A

### 🔹 Scenario 1
```rust
let s1 = String::from("rust");
let s2 = s1;
println!("{}", s1);
```
**Q: Does this compile?**
**A: No.** `s1` is moved into `s2` on line 2. `String` doesn't implement `Copy`, so `s1` becomes invalid. Using it in `println!` triggers *"value borrowed here after move"*.
**Fix:** Use `s1.clone()` on line 2, or print `s2` instead.

---

### 🔹 Scenario 2
```rust
let x = 10;
let y = x;
println!("{} {}", x, y);
```
**Q: Does this compile?**
**A: Yes.** `i32` implements `Copy`. `y = x` copies the value; both `x` and `y` remain valid independently.

---

### 🔹 Scenario 3
```rust
fn print_len(s: String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Alice");
    let len = print_len(name);
    println!("{} has length {}", name, len);
}
```
**Q: What's wrong here?**
**A:** `name` is moved into `print_len`, so it can't be used afterward in `main`.
**Fix:** Change the function signature to borrow: `fn print_len(s: &String) -> usize` and call `print_len(&name)`.

---

### 🔹 Scenario 4
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);
```
**Q: Does this compile?**
**A: No.** You cannot have two mutable references to the same data alive at the same time — this prevents data races.
**Fix:** Use `r1` fully (e.g., in its own `println!`) before creating `r2`.

---

### 🔹 Scenario 5
```rust
fn get_ref() -> &String {
    let s = String::from("temp");
    &s
}
```
**Q: What's the compiler error and why?**
**A:** "missing lifetime specifier" / dangling reference. `s` is created inside the function and dropped when the function ends — returning a reference to it would point to freed memory.
**Fix:** Return `String` (owned), not `&String`.

---

### 🔹 Scenario 6
```rust
struct Wallet {
    balance: i32,
}

fn main() {
    let wallet = Wallet { balance: 100 };
    let w2 = wallet;
    println!("{}", wallet.balance);
}
```
**Q: Does this compile?**
**A: No** — same move issue as Strings. Even though `balance: i32` is `Copy`, the *struct itself* (`Wallet`) does not derive `Copy` by default, so `wallet` moves into `w2`.
**Fix:** Add `#[derive(Copy, Clone)]` above the struct (valid since all its fields are `Copy`), or use `w2 = wallet.clone()`... but simplest here is deriving `Copy`.

---

### 🔹 Scenario 7
```rust
fn main() {
    let v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{}", first);
}
```
**Q: Does this compile?**
**A: No.** `first` is an immutable borrow of `v`. `v.push(4)` requires a mutable borrow while `first` is still "alive" (used later in `println!`) — this violates the "no mutable + immutable borrow together" rule. (Also, `push` might reallocate the Vec, which would make `first` a dangling pointer — Rust prevents this exact bug!)
**Fix:** Use `first` before calling `push`, or clone the value instead of borrowing.

---

### 🔹 Scenario 8
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = a.clone();
    println!("{}", Rc::strong_count(&a));
}
```
**Q: What gets printed, and is `a` invalidated by `.clone()`?**
**A:** Prints `2`. Unlike `String::clone()` (deep copy), `Rc::clone()` just increments the reference count — it's cheap, and `a` remains fully valid and usable.

---

### 🔹 Scenario 9
```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);
    let b1 = data.borrow_mut();
    let b2 = data.borrow_mut();
}
```
**Q: Does this compile? Does it run?**
**A:** It **compiles** (RefCell checks borrow rules at *runtime*, not compile time), but it **panics** at runtime with `"already borrowed: BorrowMutError"` because two mutable borrows are held simultaneously.
**Lesson:** `RefCell` trades compile-time safety for flexibility — you must be careful, since the compiler won't catch this.

---

### 🔹 Scenario 10 — Real-world API design
```rust
fn process(data: Vec<String>) -> Vec<String> {
    data.iter().map(|s| s.to_uppercase()).collect()
}
```
**Q: Is taking `Vec<String>` by value here a good design choice?**
**A:** It depends. If the caller no longer needs the original `data` afterward, taking ownership is fine and avoids unnecessary cloning. But if the caller *does* need `data` afterward, this forces them to clone before calling — wasteful. **Better design:** accept `&[String]` (a slice) so the function only borrows what it needs:
```rust
fn process(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_uppercase()).collect()
}
```
**Lesson:** Prefer borrowing (`&T` / `&[T]`) in function signatures unless you genuinely need ownership — it gives callers maximum flexibility.

---

## 15. Quick Recall Cheat Sheet

```
┌──────────────────────────────────────────────────────────────┐
│ OWNERSHIP RULES                                               │
│  1. Each value has ONE owner.                                  │
│  2. Only one owner at a time.                                  │
│  3. Owner out of scope → value dropped.                        │
├──────────────────────────────────────────────────────────────┤
│ MOVE vs COPY                                                   │
│  Heap data (String, Vec)   → MOVED on assignment/function call │
│  Stack-only data (i32,bool)→ COPIED automatically (Copy trait)  │
│  Want a deep copy of heap? → use .clone()                      │
├──────────────────────────────────────────────────────────────┤
│ BORROWING RULES                                                 │
│  &T      → many immutable borrows allowed at once               │
│  &mut T  → exactly ONE mutable borrow, and no other borrows      │
│  References must never outlive the data they point to           │
├──────────────────────────────────────────────────────────────┤
│ LIFETIMES                                                        │
│  'a describes RELATIONSHIPS between references' validity,        │
│  it does NOT extend how long data lives.                         │
├──────────────────────────────────────────────────────────────┤
│ SMART POINTERS                                                    │
│  Box<T>            → single owner, heap allocated                │
│  Rc<T>              → multiple owners, single-threaded            │
│  RefCell<T>          → mutability checked at RUNTIME               │
│  Rc<RefCell<T>>       → shared + mutable, single-threaded            │
│  Arc<T> / Arc<Mutex<T>> → same as above but thread-safe               │
└──────────────────────────────────────────────────────────────┘
```

### 🧠 One-Line Memory Hooks

- **Ownership** → "One deed, one owner, demolished when they leave."
- **Move** → "Handing over the only house key."
- **Copy** → "Cheap stack data — Rust just Xeroxes it."
- **Clone** → "You explicitly asked for an expensive deep copy."
- **Borrow (`&`)** → "Lending a book — you still own it."
- **`&mut`** → "Lending your *only* pen — nobody else can write until you're done."
- **Lifetime `'a`** → "A label describing how long references must remain valid together."
- **`Rc`** → "Shared subscription, single household (thread)."
- **`Arc`** → "Shared subscription, multiple households (threads)."
- **`RefCell`** → "Trust-based mutability — checked at runtime, panics if you break the rules."

---

**Practice tip:** Try to break each code example above by tweaking it — remove a `&`, add a second mutable borrow, or return a local reference — and read the compiler error. Rust's error messages are your best teacher.