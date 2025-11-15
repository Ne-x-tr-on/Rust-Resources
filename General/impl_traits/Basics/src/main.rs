// Levels 1, 2, 3 — Combined learning file
// Filename: level1_2_3_rust_constructors.rs
// This single file demonstrates practical constructor patterns for learning:
// Level 1: Basics (structs, impl, &self vs self, simple new)
// Level 2: Intermediate (multiple constructors, builder, Default, trait)
// Level 3: Advanced (fallible constructors, lifetimes, generics, async-style, trait factories)

// -----------------------------
// LEVEL 1 — BASIC
// -----------------------------
#[derive(Debug)]
struct Dad {
    age: i32,
    name: String,
    alive: bool,
}

impl Dad {
    // method that borrows &self (does not consume)
    fn greet(&self) {
        println!("Hello Dad: {}", self.name);
    }

    // status borrows &self so you can call multiple methods
    fn status(&self) {
        if self.alive {
            println!("Hey just started on Opticlass the Software we were chatting about");
        } else {
            println!("You left a legacy its working even if you are away");
        }
    }

    // tellage borrows &self
    fn tell_age(&self) {
        println!("{} age was {}", self.name, self.age);
    }

    // revive needs mutable borrow
    fn revive(&mut self) {
        self.alive = true;
        println!("Dad revived => alive = {}", self.alive);
    }

    // simple constructor (convention: new)
    fn new(age: i32, name: &str, alive: bool) -> Self {
        Self { age, name: name.to_string(), alive }
    }
}

// -----------------------------
// LEVEL 2 — INTERMEDIATE
// -----------------------------

// Multiple constructors and convenience constructors
impl Dad {
    // convenience constructor with defaults
    fn alive_default(name: &str) -> Self {
        Self { age: 50, name: name.to_string(), alive: true }
    }

    fn deceased(name: &str, age: i32) -> Self {
        Self { age, name: name.to_string(), alive: false }
    }
}

// Builder pattern for Dad (useful when there are many optional fields)
#[derive(Default)]
struct DadBuilder {
    age: Option<i32>,
    name: Option<String>,
    alive: Option<bool>,
}

impl DadBuilder {
    fn new() -> Self { Self::default() }
    fn age(mut self, age: i32) -> Self { self.age = Some(age); self }
    fn name(mut self, name: &str) -> Self { self.name = Some(name.to_string()); self }
    fn alive(mut self, alive: bool) -> Self { self.alive = Some(alive); self }

    fn build(self) -> Dad {
        Dad {
            age: self.age.unwrap_or(40),
            name: self.name.unwrap_or_else(|| "Unknown".to_string()),
            alive: self.alive.unwrap_or(true),
        }
    }
}

// Default trait for Dad
impl Default for Dad {
    fn default() -> Self {
        Self { age: 40, name: "Unknown".to_string(), alive: true }
    }
}

// Trait example: Legacy
trait Legacy {
    fn legacy_message(&self);
}

impl Legacy for Dad {
    fn legacy_message(&self) {
        if self.alive {
            println!("Your legacy is still in progress!");
        } else {
            println!("Your legacy lives on!");
        }
    }
}

// -----------------------------
// LEVEL 3 — ADVANCED
// -----------------------------
use std::error::Error;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
enum DadError {
    InvalidAge,
    EmptyName,
}

impl fmt::Display for DadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DadError::InvalidAge => write!(f, "Invalid age"),
            DadError::EmptyName => write!(f, "Empty name"),
        }
    }
}

impl Error for DadError {}

// Fallible constructor that returns Result<Self, Error>
impl Dad {
    fn try_new(age: i32, name: &str, alive: bool) -> Result<Self, DadError> {
        if age <= 0 { return Err(DadError::InvalidAge); }
        if name.is_empty() { return Err(DadError::EmptyName); }
        Ok(Self { age, name: name.to_string(), alive })
    }
}

// Generic constructor using Into<String> so callers can pass &str or String
impl Dad {
    fn with_name<T: Into<String>>(age: i32, name: T, alive: bool) -> Self {
        Self { age, name: name.into(), alive }
    }
}

// Lifetimes / zero-copy: a struct that borrows a name slice
struct DadRef<'a> {
    age: i32,
    name: &'a str,
    alive: bool,
}

impl<'a> DadRef<'a> {
    fn new(name: &'a str, age: i32, alive: bool) -> Result<Self, DadError> {
        if age <= 0 { return Err(DadError::InvalidAge); }
        if name.is_empty() { return Err(DadError::EmptyName); }
        Ok(Self { age, name, alive })
    }
}

// Async-style constructor (illustrative: synchronous function here to avoid adding runtime)
impl Dad {
    // In real async code you'd write `async fn init_from_remote(...) -> Result<Self, Error>`
    fn init_from_remote(name: &str) -> Result<Self, DadError> {
        // pretend we called some remote init and got age
        if name.is_empty() { return Err(DadError::EmptyName); }
        Ok(Self { age: 55, name: name.to_string(), alive: true })
    }
}

// Factory returning trait objects (different concrete types behind a trait)
trait Speaker { fn speak(&self); }

struct LiveSpeaker; struct SilentSpeaker;
impl Speaker for LiveSpeaker { fn speak(&self) { println!("I'm alive and speaking"); } }
impl Speaker for SilentSpeaker { fn speak(&self) { println!("...silence..."); } }

fn speaker_factory(alive: bool) -> Box<dyn Speaker> {
    if alive { Box::new(LiveSpeaker) } else { Box::new(SilentSpeaker) }
}

// Self-referencing with Arc example (simple parent pointer)
struct Node {
    name: String,
    parent: Option<Arc<Node>>,
}

impl Node {
    fn new_root(name: &str) -> Arc<Self> { Arc::new(Self { name: name.to_string(), parent: None }) }
    fn new_child(parent: &Arc<Node>, name: &str) -> Arc<Self> { Arc::new(Self { name: name.to_string(), parent: Some(parent.clone()) }) }
}

// -----------------------------
// Demonstration main() covering levels 1-3
// -----------------------------
fn main() -> Result<(), Box<dyn Error>> {
    // Level 1: basics
    let mut d = Dad::new(55, "David Kamau", false);
    d.status();
    d.tell_age();
    d.revive();

    // Level 2: multiple constructors & builder
    let d2 = Dad::alive_default("Peter");
    d2.greet();

    let built = DadBuilder::new().name("BuilderKamau").age(45).alive(false).build();
    println!("Built Dad: {:?}", built);

    let default_dad = Dad::default();
    default_dad.greet();

    default_dad.legacy_message();

    // Level 3: fallible constructor
    match Dad::try_new(0, "", true) {
        Ok(_) => println!("Shouldn't succeed"),
        Err(e) => println!("Expected error: {}", e),
    }

    let g1 = speaker_factory(d.is_alive());
    g1.speak();

    // lifetimes (zero-copy)
    let raw = String::from("ZeroCopyName");
    let dref = DadRef::new(&raw, 44, true)?;
    println!("DadRef: {} - {}", dref.name, dref.age);

    // self referencing
    let root = Node::new_root("root");
    let child = Node::new_child(&root, "child");
    println!("child has parent: {}", child.parent.is_some());

    // generic constructor
    let g2 = Dad::with_name(60, String::from("GenericKamau"), true);
    println!("Generic dad name: {}", g2.name);

    // async-style init (sync here for demo)
    let remote = Dad::init_from_remote("RemoteKamau")?;
    remote.greet();

    Ok(())
}
