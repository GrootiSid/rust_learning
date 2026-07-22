# 🦀 Rust Lifetimes — Complete Guide (Basic → Advanced)

> **Goal:** Go from "what is `'a`?" to confidently debugging HRTB and async lifetime errors in a real backend service.
> Every section: **Concept → Code → Why it matters → Recall trick**.

---

## 📌 Table of Contents

1. [The Core Idea (Recap)](#1-the-core-idea-recap)
2. [Lifetime Elision](#2-lifetime-elision)
3. [Explicit Lifetimes](#3-explicit-lifetimes)
4. [Lifetime Bounds](#4-lifetime-bounds)
5. [The `'static` Lifetime](#5-the-static-lifetime)
6. [Generic Lifetime Parameters](#6-generic-lifetime-parameters)
7. [Lifetime Subtyping](#7-lifetime-subtyping)
8. [Lifetime Variance](#8-lifetime-variance)
9. [Higher-Ranked Trait Bounds (HRTB)](#9-higher-ranked-trait-bounds-hrtb)
10. [Async Lifetime Challenges](#10-async-lifetime-challenges)
11. [Real Backend Project: Zero-Copy Request Router](#11-real-backend-project-zero-copy-request-router)
12. [Scenario-Based Q&A](#12-scenario-based-qa)
13. [Quick Recall Cheat Sheet](#13-quick-recall-cheat-sheet)

---

## 1. The Core Idea (Recap)

A lifetime is **not** a duration you control — it's a **name the compiler gives to a region of code** where a reference is guaranteed valid. Lifetimes exist purely to answer one question:

> *"Will this reference still point to valid memory everywhere it's used?"*

```rust
fn main() {
    let r;                // ---------+-- 'a (r's lifetime)
    {                      //          |
        let x = 5;         // -+-- 'b  |
        r = &x;            //  |       |
    }                      // -+       |  <- x dropped here
    // println!("{}", r);  // ❌ ERROR: x doesn't live long enough
}                          // ---------+
```

`'b` (x's lifetime) is shorter than `'a` (r's lifetime), so borrowing `x` into `r` is rejected — `r` would dangle.

> 💡 **Recall trick:** "A lifetime is a promise: *this reference won't outlive the data it points to.* The compiler is just fact-checking that promise."

---

## 2. Lifetime Elision

Most functions with references **don't need explicit lifetime annotations** because the compiler applies 3 mechanical rules first, and only asks you to write `'a` if those rules leave it ambiguous.

### The 3 Elision Rules

```
Rule 1: Each elided lifetime in the parameters gets its own lifetime parameter.
        fn foo(x: &str, y: &str)  →  fn foo<'a, 'b>(x: &'a str, y: &'b str)

Rule 2: If there's exactly ONE input lifetime, it's assigned to ALL elided output lifetimes.
        fn foo(x: &str) -> &str  →  fn foo<'a>(x: &'a str) -> &'a str

Rule 3: If one of the params is `&self` or `&mut self`, its lifetime is
        assigned to all elided output lifetimes.
        fn foo(&self, x: &str) -> &str  →  fn foo<'a, 'b>(&'a self, x: &'b str) -> &'a str
```

### Examples

```rust
// Elided — compiles fine, Rule 2 applies
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap()
}

// Equivalent explicit form:
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    s.split(' ').next().unwrap()
}
```

```rust
struct Parser {
    input: String,
}

impl Parser {
    // Elided — Rule 3 applies: &self's lifetime flows to the return type
    fn current(&self) -> &str {
        &self.input[0..1]
    }
}
```

### When elision FAILS (you must write `'a`)

```rust
// ❌ ERROR: two input references, no &self — Rule 2/3 don't apply
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// ✅ Must be explicit — the compiler can't guess if the output ties to x or y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

> 💡 **Recall trick:** "Elision = autocomplete for lifetimes. One input reference, or a `&self` → Rust guesses correctly. Two or more unrelated inputs → you must spell it out."

---

## 3. Explicit Lifetimes

### In Functions

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
`'a` here means: *"the returned reference is valid for at most the shorter of `x`'s and `y`'s lifetimes."*

### In Structs

Structs can't hold references without declaring the lifetime — the struct **cannot outlive** the data it borrows.

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part // ties to 'a via elision Rule 3
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence };
    println!("{}", excerpt.part);
} // excerpt must be dropped before/with `novel` — enforced by 'a
```

### In Enums

```rust
enum Token<'a> {
    Word(&'a str),
    Number(i64),
}
```

### Multiple Lifetime Parameters

```rust
struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}
```
Use two distinct lifetimes when the two references have **genuinely unrelated** lifespans — using one `'a` for both would incorrectly force them to be tied together.

> 💡 **Recall trick:** "A struct holding a reference is basically a promise: *'I will never be used after the thing I'm pointing to is gone.'* The lifetime parameter is where that promise is written down."

---

## 4. Lifetime Bounds

Lifetime bounds constrain **generic types**, not just references — they say *"this generic type must not contain any reference that lives shorter than `'a`."*

### `T: 'a` — "T outlives 'a"

```rust
struct Wrapper<'a, T: 'a> {
    value: &'a T,
}
```
Since Rust 2018, this is inferred automatically in most cases, but it's important when writing generic code manually (e.g., in trait definitions).

### Combining Trait Bounds + Lifetime Bounds

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

### `T: 'static` bound

```rust
fn process<T: 'static>(value: T) {
    // value is guaranteed to not contain any non-'static references
}
```
This is common in generic APIs (e.g., `tokio::spawn`) that need to guarantee the value can safely live indefinitely (e.g., moved to another thread).

> 💡 **Recall trick:** "`T: 'a` isn't about T's own lifetime — it's a guarantee that any references *hidden inside* T live at least as long as `'a`."

---

## 5. The `'static` Lifetime

`'static` means: *the reference is valid for the entire remaining duration of the program.*

### Where it appears naturally

```rust
let s: &'static str = "I am baked into the binary"; // string literals are always 'static
```

### `'static` in generic bounds (a DIFFERENT meaning!)

```rust
fn spawn_task<T: 'static + Send>(task: T) {
    // T must not borrow anything with a limited lifetime
}
```
Here, `T: 'static` doesn't mean "T lives forever" — it means **"T owns all its data, or only borrows `'static` data."** This is why `tokio::spawn` requires `'static` futures: the spawned task might outlive the function that created it.

### The Common Beginner Mistake

```rust
fn get_str() -> &'static str {
    let s = String::from("hello");
    &s // ❌ ERROR: `s` doesn't live for 'static, it's dropped at function end
}
```
**Fix:** return an owned `String`, or use a real `'static` source (like a string literal or `Box::leak`, rarely justified).

> 💡 **Recall trick:** "`'static` on a reference = 'lives forever, like a string literal.' `'static` on a generic bound = 'owns its data, no borrowed short-lived references inside.' Don't confuse the two."

---

## 6. Generic Lifetime Parameters

Lifetimes are generic parameters, just like types — they can appear on functions, structs, traits, and impls, and can be combined with type generics.

```rust
struct Cache<'a, T> {
    source: &'a T,
    hits: u32,
}

impl<'a, T> Cache<'a, T> {
    fn new(source: &'a T) -> Self {
        Cache { source, hits: 0 }
    }
}
```

### Multiple lifetimes that relate to each other

```rust
struct Context<'a> {
    data: &'a str,
}

struct Parser<'ctx, 'a: 'ctx> {  // 'a must outlive 'ctx
    context: &'ctx Context<'a>,
}
```
This says: *"the `Context` we're pointing to (`'ctx`) must not outlive the string data it itself borrows (`'a`)."* This is **lifetime subtyping**, covered next.

> 💡 **Recall trick:** "Lifetimes are just another kind of generic parameter — think `<'a, T>` the same way you think `<T, U>`."

---

## 7. Lifetime Subtyping

`'a: 'b` reads as **"`'a` outlives `'b`"** (or "`'a` is a subtype of `'b`") — meaning anywhere a `'b` reference is expected, an `'a` reference can be used, because `'a` is guaranteed to be valid for at least as long.

```rust
struct Parser<'s, 'c> where 's: 'c {  // 's must outlive 'c
    source: &'s str,
    cursor: &'c mut usize,
}
```

### Practical example: combining short and long-lived data

```rust
fn choose_longer_lived<'long: 'short, 'short>(
    long: &'long str,
    _short: &'short str,
) -> &'short str {
    long // ✅ OK — 'long outlives 'short, so a &'long can be used as &'short
}
```

> 💡 **Recall trick:** "`'a: 'b` = '`'a` is at least as long-lived as `'b`.' Think of it like a type hierarchy: a longer lifetime can always substitute for a shorter one, never the reverse."

---

## 8. Lifetime Variance

Variance determines whether a subtyping relationship on lifetimes "passes through" to the compound type built from them. This is advanced, but crucial for understanding some confusing compiler errors.

### Covariant (`&'a T`) — most common

If `'long: 'short`, then `&'long T` can be used wherever `&'short T` is expected. **Shared references are covariant** in their lifetime.

```rust
fn shorten<'long: 'short, 'short>(r: &'long i32) -> &'short i32 {
    r // ✅ fine — covariance allows a longer-lived ref to act as a shorter-lived one
}
```

### Invariant (`&'a mut T`) — mutable references

`&mut T` is **invariant** over `T` — meaning `&mut &'long str` and `&mut &'short str` are NOT interchangeable, even if `'long: 'short`. This prevents unsound mutation.

```rust
fn bad_example<'long: 'short, 'short>(
    r: &mut &'long str,
    s: &'short str,
) {
    // If this were allowed, we could stuff a short-lived reference
    // into a slot that's supposed to hold a long-lived one:
    // *r = s; // ❌ this is REJECTED — invariance protects against this
}
```

### Contravariant (function arguments) — rare in practice

Function types are contravariant in their argument lifetimes. This mostly matters for `fn` pointer types and closures, and is rarely something you write by hand — but it's why HRTB (`for<'a>`) exists (next section).

| Type | Variance over `'a` |
|---|---|
| `&'a T` | Covariant |
| `&'a mut T` | Covariant in `'a`, invariant in `T` |
| `Fn(&'a T)` (argument position) | Contravariant |
| `Cell<T>`, `RefCell<T>` | Invariant in `T` |

> 💡 **Recall trick:** "Shared refs are flexible (covariant) — a longer-lived one can stand in for a shorter one. Mutable refs are strict (invariant) — no substitution allowed, or you could sneak a short-lived value into a long-lived slot."

---

## 9. Higher-Ranked Trait Bounds (HRTB)

Sometimes a function needs to accept a closure or trait object that works **for any lifetime**, not just one specific lifetime chosen by the caller. This is what `for<'a>` expresses.

### The Problem Without HRTB

```rust
// This would only work for ONE specific lifetime, chosen at the call site —
// too restrictive for a closure that must work with many different borrowed strings.
fn apply<'a, F>(f: F, s: &'a str) -> &'a str
where
    F: Fn(&'a str) -> &'a str,
{
    f(s)
}
```

### With HRTB — works for ANY lifetime

```rust
fn apply_hrtb<F>(f: F, s: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str, // "for ALL lifetimes 'a"
{
    f(s)
}

fn main() {
    let result = apply_hrtb(|x| x, "hello");
    println!("{}", result);
}
```

**In practice, this is usually elided too!** Rust automatically infers `for<'a>` for simple closure bounds:

```rust
fn apply_simple<F>(f: F, s: &str) -> &str
where
    F: Fn(&str) -> &str, // implicitly for<'a> Fn(&'a str) -> &'a str
{
    f(s)
}
```

### Where you'll actually SEE `for<'a>` explicitly

Most commonly with trait objects and complex generic bounds:

```rust
trait Filter {
    fn matches<'a>(&self, input: &'a str) -> bool;
}

fn run_filter(f: &dyn for<'a> Fn(&'a str) -> bool, input: &str) -> bool {
    f(input)
}
```

> 💡 **Recall trick:** "HRTB = 'this closure/function must work no matter WHICH lifetime you throw at it, not just one fixed lifetime.' Think of `for<'a>` as a universal quantifier: 'for all `'a`', like `∀` in math."

---

## 10. Async Lifetime Challenges

Async Rust is where lifetimes get genuinely hard, because `async fn` desugars into a **state machine (a `Future`)** that must store all data used across `.await` points — including borrowed references.

### Challenge 1: `async fn` borrowing `&self`

```rust
struct Server {
    name: String,
}

impl Server {
    // Desugars to: fn handle<'a>(&'a self) -> impl Future<Output = ()> + 'a
    async fn handle(&self) {
        println!("Handling on {}", self.name);
        some_async_work().await;
    }
}
```
The returned `Future` **borrows `self`**, so it can't outlive the `Server`. This is usually fine — until you try to `tokio::spawn` it.

### Challenge 2: Spawning requires `'static`

```rust
async fn broken(server: &Server) {
    tokio::spawn(server.handle()); // ❌ ERROR: future may not live long enough
}
```
`tokio::spawn` requires a `'static` future (it might run after the current function returns), but `server.handle()` borrows `server`, which isn't `'static`.

**Fix 1 — Clone/own the data before spawning:**
```rust
async fn fixed(server: Arc<Server>) {
    let server_clone = Arc::clone(&server);
    tokio::spawn(async move {
        server_clone.handle().await; // owns an Arc, satisfies 'static
    });
}
```

**Fix 2 — Restructure to avoid borrowing across the spawn boundary:**
```rust
async fn fixed_v2(name: String) {
    tokio::spawn(async move {
        println!("Handling on {}", name); // owns `name`, no borrow needed
    });
}
```

### Challenge 3: Lifetimes in trait methods returning futures (pre-async-trait-in-traits)

```rust
use std::future::Future;

trait Repository {
    // Older stable Rust required boxing: HRTB + async don't mix simply
    fn find<'a>(&'a self, id: u32) -> std::pin::Pin<Box<dyn Future<Output = Option<String>> + 'a>>;
}
```
Modern Rust (1.75+) supports `async fn` directly in traits, but the returned future is still tied to `&'a self`, so the same "must own data before spawning" rules apply.

### Challenge 4: Self-referential futures

```rust
async fn process() {
    let data = String::from("hello");
    let reference = &data;         // borrows local `data`
    some_async_call(reference).await; // the Future stores `reference` across the .await
}
```
This works *within* one function because the compiler tracks it precisely — but it's exactly this kind of "struct holding a reference to its own field" pattern that makes `Future`s need `Pin` under the hood, preventing the data from being moved in memory while borrowed.

> 💡 **Recall trick:** "`async fn` = a struct in disguise, and that struct might hold references. Anything you `tokio::spawn` must be `'static` because the spawned task can run *after* your function returns — so it can't borrow local, short-lived data. When in doubt: clone, `Arc`, or use `move` closures to hand over ownership."

---

## 11. Real Backend Project: Zero-Copy Request Router

A minimal HTTP-style backend router demonstrating: elision, explicit lifetimes, struct lifetimes, HRTB handler dispatch, and the async `'static` requirement — all together.

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// ---------- 1. Zero-copy request parsing (explicit lifetimes) ----------

/// Borrows directly from the raw request bytes — no allocation for parsing.
struct Request<'a> {
    method: &'a str,
    path: &'a str,
    headers: HashMap<&'a str, &'a str>,
}

impl<'a> Request<'a> {
    /// Parses "GET /users/42\nHost: localhost\n\n" style raw input.
    /// Elision Rule 2 applies: one input lifetime -> flows to output.
    fn parse(raw: &'a str) -> Option<Request<'a>> {
        let mut lines = raw.lines();
        let first_line = lines.next()?;
        let mut parts = first_line.split_whitespace();
        let method = parts.next()?;
        let path = parts.next()?;

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((k, v)) = line.split_once(": ") {
                headers.insert(k, v);
            }
        }

        Some(Request { method, path, headers })
    }

    // Elision Rule 3: &self's lifetime flows to the return type.
    fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).copied()
    }
}

// ---------- 2. Handler trait using HRTB ----------

/// The router needs handlers that work for ANY request lifetime, not just one
/// fixed lifetime chosen when the handler was registered — hence `for<'a>`.
trait Handler: Send + Sync {
    fn call(&self, req: &Request<'_>) -> String;
}

// A closure automatically satisfies `for<'a> Fn(&Request<'a>) -> String`
// thanks to elision, so we can store boxed closures as trait objects:
struct FnHandler<F>(F)
where
    F: for<'a> Fn(&Request<'a>) -> String + Send + Sync;

impl<F> Handler for FnHandler<F>
where
    F: for<'a> Fn(&Request<'a>) -> String + Send + Sync,
{
    fn call(&self, req: &Request<'_>) -> String {
        (self.0)(req)
    }
}

// ---------- 3. The router owns 'static data — no borrowed handlers ----------

struct Router {
    // Keys and handlers must be 'static (owned) because the Router itself
    // is going to be shared across async tasks via Arc.
    routes: HashMap<String, Box<dyn Handler>>,
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn register<F>(&mut self, path: &str, handler: F)
    where
        F: for<'a> Fn(&Request<'a>) -> String + Send + Sync + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(FnHandler(handler)));
    }

    // The request borrows from `raw` (short-lived), but the Router itself is
    // long-lived ('static, shared via Arc) — this mix is exactly what
    // lifetime elision + HRTB make safe and ergonomic.
    fn dispatch(&self, raw: &str) -> String {
        match Request::parse(raw) {
            Some(req) => match self.routes.get(req.path) {
                Some(handler) => handler.call(&req),
                None => "404 Not Found".to_string(),
            },
            None => "400 Bad Request".to_string(),
        }
    }
}

// ---------- 4. Async server state: shared, owned, 'static ----------

struct AppState {
    request_count: Mutex<u32>,
}

async fn handle_connection(state: Arc<AppState>, router: Arc<Router>, raw_request: String) {
    // `raw_request` is OWNED (String, not &str) specifically so this async
    // function has no borrowed data crossing the .await point below —
    // satisfying the 'static requirement for tokio::spawn.
    {
        let mut count = state.request_count.lock().await;
        *count += 1;
    } // lock released before dispatch — avoids holding it across dispatch

    let response = router.dispatch(&raw_request); // short-lived borrow, resolved before return
    println!("Response: {}", response);
}

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    router.register("/health", |_req| "OK".to_string());

    router.register("/echo", |req: &Request<'_>| {
        format!("Method: {}, Path: {}", req.method, req.path)
    });

    let router = Arc::new(router);
    let state = Arc::new(AppState { request_count: Mutex::new(0) });

    let sample_requests = vec![
        "GET /health\nHost: localhost\n\n".to_string(),
        "GET /echo\nHost: localhost\n\n".to_string(),
    ];

    let mut handles = vec![];
    for raw in sample_requests {
        let state = Arc::clone(&state);
        let router = Arc::clone(&router);
        // `raw` (String) is MOVED into the async block — owned data,
        // safe to spawn because nothing here borrows short-lived data.
        handles.push(tokio::spawn(async move {
            handle_connection(state, router, raw).await;
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    println!("Total requests handled: {}", *state.request_count.lock().await);
}
```

### What this project teaches, mapped to sections above:

| Concept | Where it appears |
|---|---|
| Explicit struct lifetimes | `Request<'a>` borrows directly from raw request text — zero-copy parsing |
| Lifetime elision | `Request::parse`, `Request::header` — compiler infers `'a` automatically |
| HRTB (`for<'a>`) | `Handler` trait & `register()` — handlers must work for any request lifetime |
| `'static` bound | `F: ... + 'static` on `register()` — handlers can't borrow short-lived data |
| `'static` for async spawn | `raw_request: String` (owned) passed into `tokio::spawn`, not `&str` |
| `Arc` for shared ownership | `Router` and `AppState` shared across spawned tasks |
| Avoiding borrow-across-await | Mutex lock scoped with `{ }` and released before `.await` inside `dispatch` |

---

## 12. Scenario-Based Q&A

### 🔹 Scenario 1
```rust
fn get_first(s: &str) -> &str {
    s.split(',').next().unwrap()
}
```
**Q: No lifetime annotations here — does it compile?**
**A: Yes.** Elision Rule 2 applies: exactly one input lifetime, so the compiler assigns it to the output automatically. No need to write `<'a>`.

---

### 🔹 Scenario 2
```rust
fn pick(a: &str, b: &str, use_a: bool) -> &str {
    if use_a { a } else { b }
}
```
**Q: Does this compile?**
**A: No.** Two input references, no `&self` — elision Rules 2 and 3 don't apply, so the compiler can't infer the output's lifetime.
**Fix:**
```rust
fn pick<'a>(a: &'a str, b: &'a str, use_a: bool) -> &'a str {
    if use_a { a } else { b }
}
```

---

### 🔹 Scenario 3
```rust
struct Config<'a> {
    name: &'a str,
}

fn make_config() -> Config<'static> {
    let name = String::from("dynamic"); // owned, local
    Config { name: &name } // ❌ ERROR
}
```
**Q: Why does this fail?**
**A:** The function promises to return `Config<'static>` — a config whose `name` lives forever — but `name` is a local `String` dropped at the end of the function. There's no way to satisfy `'static` here.
**Fix:** Either use a real `'static` source (`Config { name: "dynamic" }`, a string literal), or change the return type to `Config<'_>` tied to an input parameter instead of promising `'static`.

---

### 🔹 Scenario 4
```rust
fn shorten<'long: 'short, 'short>(long_ref: &'long str) -> &'short str {
    long_ref
}
```
**Q: Why does this compile even though `'long` and `'short` look different?**
**A:** Because `&'a T` is **covariant** over `'a`, and the bound `'long: 'short` establishes that `'long` outlives `'short`. A reference valid for longer can always be used somewhere expecting a shorter validity guarantee. This is lifetime subtyping in action.

---

### 🔹 Scenario 5
```rust
fn assign<'long: 'short, 'short>(r: &mut &'long str, s: &'short str) {
    *r = s;
}
```
**Q: Does this compile?**
**A: No.** `&mut T` is **invariant** over `T`. Even though `'long: 'short`, you cannot assign a `&'short str` into a slot typed `&mut &'long str` — that would let a short-lived reference escape into a context expecting a long-lived one, creating a potential dangling reference once `'short` ends. Invariance exists specifically to block this.

---

### 🔹 Scenario 6
```rust
fn apply<F: Fn(&str) -> &str>(f: F, input: &str) -> String {
    f(input).to_string()
}
```
**Q: Is this using HRTB, and do I need to write `for<'a>` explicitly?**
**A:** Yes, it's using HRTB — but you don't have to write it. `F: Fn(&str) -> &str` is automatically elided/expanded by the compiler into `F: for<'a> Fn(&'a str) -> &'a str`. Explicit `for<'a>` syntax is only needed in more complex cases (e.g., trait objects, multiple unrelated bound lifetimes).

---

### 🔹 Scenario 7
```rust
struct Server { name: String }

impl Server {
    async fn greet(&self) -> String {
        format!("Hello from {}", self.name)
    }
}

async fn run(server: &Server) {
    tokio::spawn(server.greet()); // ❌ ERROR
}
```
**Q: What's the exact problem, and how do you fix it while keeping `spawn`?**
**A:** `server.greet()` desugars to a `Future` that borrows `&'a self`, tied to the lifetime of `server`. `tokio::spawn` requires `'static` futures because the spawned task could run after `run()` returns, when `server` might already be dropped.
**Fix:** wrap `Server` in `Arc<Server>`, clone the `Arc` before spawning, and move the clone into the async block:
```rust
async fn run(server: Arc<Server>) {
    let server = Arc::clone(&server);
    tokio::spawn(async move {
        let greeting = server.greet().await;
        println!("{}", greeting);
    });
}
```

---

### 🔹 Scenario 8
```rust
struct Cache<'a> {
    data: &'a Vec<String>,
}

fn build_cache() -> Cache<'static> {
    let v = vec!["a".to_string(), "b".to_string()];
    Cache { data: &v } // ❌ ERROR
}
```
**Q: Same root problem as Scenario 3 — what's the general lesson?**
**A:** Any struct that claims a `'static` lifetime for borrowed data is promising the data lives for the entire program. Local variables never satisfy this. The general lesson: **`'static` structs should hold owned data (or references to genuinely global/leaked data), not references to locals** — if you need a self-contained, freely-passable struct, prefer owning the data (`Vec<String>` instead of `&'a Vec<String>`).

---

### 🔹 Scenario 9
```rust
fn make_adder<'a>(x: &'a i32) -> impl Fn(i32) -> i32 + 'a {
    move |y| *x + y
}
```
**Q: Why is `+ 'a` needed on the return type here?**
**A:** `impl Trait` return types default to requiring `'static` unless you constrain them. Since the closure captures `x: &'a i32`, the returned closure can only be valid as long as `'a` — so you must explicitly bound the `impl Fn(...)` with `+ 'a` to tell the compiler "this returned closure is only guaranteed to live as long as the borrowed data it captured."

---

### 🔹 Scenario 10
```rust
trait Repository: Send + Sync {
    async fn find(&self, id: u32) -> Option<String>;
}

struct InMemoryRepo {
    data: Arc<Mutex<HashMap<u32, String>>>,
}

impl Repository for InMemoryRepo {
    async fn find(&self, id: u32) -> Option<String> {
        self.data.lock().await.get(&id).cloned()
    }
}

async fn spawn_lookup(repo: Arc<dyn Repository>, id: u32) {
    tokio::spawn(async move {
        let result = repo.find(id).await;
        println!("{:?}", result);
    });
}
```
**Q: Why does this compile cleanly despite `find` borrowing `&self`?**
**A:** Because `repo` is `Arc<dyn Repository>` — an **owned, reference-counted, `'static`-compatible handle**. The `async move` block takes ownership of the `Arc` clone, not a borrow of `repo` itself, so nothing short-lived crosses into the spawned task. `.cloned()` also ensures the returned `String` is owned, not tied to the mutex guard's lifetime. This is the standard pattern for async trait methods used across spawned tasks: **wrap shared state in `Arc`, clone before spawning, return owned data from async methods.**

---

## 13. Quick Recall Cheat Sheet

```
┌────────────────────────────────────────────────────────────────┐
│ ELISION RULES                                                    │
│  1. Each elided input ref gets its own lifetime.                  │
│  2. One input lifetime  → assigned to all elided outputs.          │
│  3. &self / &mut self   → its lifetime assigned to elided outputs. │
├────────────────────────────────────────────────────────────────┤
│ EXPLICIT LIFETIMES                                                 │
│  Needed when: 2+ unrelated input refs, or struct holding a ref.     │
│  'a on a struct = "this struct cannot outlive the data it borrows." │
├────────────────────────────────────────────────────────────────┤
│ BOUNDS                                                              │
│  T: 'a       → any refs INSIDE T must live at least as long as 'a.   │
│  T: 'static  → T owns its data / contains no short-lived borrows.    │
├────────────────────────────────────────────────────────────────┤
│ 'static                                                               │
│  On a reference → valid for the WHOLE program (string literals).      │
│  On a generic bound → "owns its data, no limited-lifetime refs inside."│
├────────────────────────────────────────────────────────────────┤
│ SUBTYPING & VARIANCE                                                    │
│  'long: 'short   → 'long is valid at least as long as 'short.            │
│  &'a T           → covariant (longer can substitute for shorter).         │
│  &'a mut T       → invariant over T (no substitution allowed).             │
├────────────────────────────────────────────────────────────────┤
│ HRTB                                                                      │
│  for<'a> Fn(&'a str) -> &'a str  → works for ANY lifetime, not just one.   │
│  Usually elided automatically for simple Fn/FnMut/FnOnce bounds.           │
├────────────────────────────────────────────────────────────────┤
│ ASYNC + LIFETIMES                                                          │
│  async fn borrowing &self → Future tied to that borrow's lifetime.          │
│  tokio::spawn requires 'static → clone into Arc, use `async move`,          │
│  return OWNED data from async methods, avoid holding refs across .await.     │
└────────────────────────────────────────────────────────────────┘
```

### 🧠 One-Line Memory Hooks

- **Elision** → "The compiler's autocomplete — works for 1 input ref or `&self`."
- **Explicit `'a`** → "Needed when the compiler has more than one reasonable guess."
- **`T: 'a` bound** → "Nothing hiding inside T is shorter-lived than `'a`."
- **`'static` on a ref** → "Lives forever, like a string literal."
- **`'static` on a bound** → "Owns its data — no borrowed shortcuts allowed."
- **`'long: 'short`** → "`'long` can always cover for `'short`, never the reverse."
- **Covariant `&T`** → "Flexible — a longer-lived ref slots in anywhere a shorter one is expected."
- **Invariant `&mut T`** → "Strict — no substitutions, protects against smuggling short-lived data in."
- **HRTB `for<'a>`** → "Works for ALL lifetimes, not one fixed lifetime — the `∀` of Rust."
- **Async + spawn** → "If it might outlive the current function, it must own its data (`'static`)."

---

**Practice tip:** Take the backend project, break one lifetime bound at a time (remove `'static` from `register`, try spawning a task that borrows `raw_request` instead of owning it, swap `Arc::clone` for a plain reference) — then read what the compiler says. Every error message names the exact rule you just violated.