// Level 4 — Professional constructor patterns in a single Rust codebase
// Filename: level4_rust_constructors.rs
// This single file demonstrates many advanced constructor patterns and system designs:
// - Dependency Injection (trait-based)
// - Abstract Factory (returning trait objects)
// - Constructor macros (generate `new` / `try_new`)
// - Fallible constructors (Result<Self, Error>)
// - Async-style constructors (illustrative, uses mocked async functions)
// - Builder pattern
// - Hidden fields + public constructor (encapsulation)
// - State machine with constructors that set the initial state
// - Zero-copy constructor using lifetimes
// - Self-referencing via Arc example
// - Factory returning Box<dyn Trait>

use std::error::Error;
use std::fmt;
use std::sync::Arc;

// -----------------------------
// Utility: a small error type
// -----------------------------
#[derive(Debug)]
enum AppError {
    InvalidAge,
    EmptyName,
    IoError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidAge => write!(f, "Invalid age provided"),
            AppError::EmptyName => write!(f, "Name cannot be empty"),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for AppError {}

// -----------------------------
// 1) Macro that generates constructors
// -----------------------------
// This macro creates an impl block with new() and try_new()
macro_rules! make_constructors {
    ($t:ident, $name_ty:ty) => {
        impl $t {
            pub fn new(age: i32, name: $name_ty) -> Self {
                Self {
                    age,
                    name: name.into(),
                    alive: true,
                }
            }

            pub fn try_new(age: i32, name: $name_ty) -> Result<Self, AppError> {
                if age <= 0 { return Err(AppError::InvalidAge); }
                let name_str: String = name.into();
                if name_str.is_empty() { return Err(AppError::EmptyName); }
                Ok(Self { age, name: name_str, alive: true })
            }
        }
    };
}

// -----------------------------
// 2) The core struct (hidden fields example)
// -----------------------------
pub struct Dad {
    age: i32,
    name: String,
    alive: bool,
}

// Use the macro to generate new() and try_new() accepting types convertible into String
make_constructors!(Dad, impl Into<String>);
// Note: the `impl Into<String>` in macro usage above is illustrative — macros don't
// accept `impl` tokens like that directly. In real code you'd call the macro with
// a concrete type or adapt the macro accordingly. Here the macro invocation is
// intentionally simplified for a single-file didactic example.

// Because the struct's fields are private (no `pub`), external code must use
// provided constructors and methods. This enforces invariants.

impl Dad {
    // convenience constructor with defaults
    pub fn deceased(name: &str, age: i32) -> Self {
        Self { age, name: name.to_string(), alive: false }
    }

    pub fn greet(&self) {
        println!("Hello {} (age: {})", self.name, self.age);
    }

    pub fn is_alive(&self) -> bool { self.alive }
}

// -----------------------------
// 3) Builder Pattern (for large/optional fields)
// -----------------------------
#[derive(Debug, Default)]
pub struct DadBuilder {
    age: Option<i32>,
    name: Option<String>,
    alive: Option<bool>,
}

impl DadBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn age(mut self, age: i32) -> Self { self.age = Some(age); self }
    pub fn name(mut self, name: &str) -> Self { self.name = Some(name.to_string()); self }
    pub fn alive(mut self, alive: bool) -> Self { self.alive = Some(alive); self }

    // Fallible build
    pub fn build(self) -> Result<Dad, AppError> {
        let age = self.age.ok_or(AppError::InvalidAge)?;
        if age <= 0 { return Err(AppError::InvalidAge); }
        let name = self.name.ok_or(AppError::EmptyName)?;
        if name.is_empty() { return Err(AppError::EmptyName); }
        Ok(Dad { age, name, alive: self.alive.unwrap_or(true) })
    }
}

// -----------------------------
// 4) Zero-copy constructor with lifetimes
// -----------------------------
// A DadRef struct borrows the name instead of owning it. This is useful for
// zero-copy parsing/serialization pipelines.
pub struct DadRef<'a> {
    pub age: i32,
    pub name: &'a str,
    pub alive: bool,
}

impl<'a> DadRef<'a> {
    pub fn new(name: &'a str, age: i32, alive: bool) -> Result<Self, AppError> {
        if age <= 0 { return Err(AppError::InvalidAge); }
        if name.is_empty() { return Err(AppError::EmptyName); }
        Ok(Self { age, name, alive })
    }
}

// -----------------------------
// 5) Async-style constructor (mocked)
// -----------------------------
// In production you'd use real async code (Tokio, async-std, SQLx, etc.)
// Here we mock an async initializer using a synchronous function for simplicity.
impl Dad {
    // Example 'async' initializer (synchronous for this example)
    // Real usage: `async fn init_from_remote(...) -> Result<Self, AppError>`
    pub fn init_from_service(name: &str) -> Result<Self, AppError> {
        // pretend to call a remote service, then build
        if name.is_empty() { return Err(AppError::EmptyName); }
        Ok(Dad { age: 42, name: name.to_string(), alive: true })
    }
}

// -----------------------------
// 6) Dependency Injection via traits
// -----------------------------
// A database trait that our constructors or factories can depend on
#[async_trait::async_trait]
trait Database: Send + Sync {
    async fn get_dad_by_id(&self, id: u64) -> Result<Dad, AppError>;
}

// We'll provide a mock in-memory DB that implements Database.
struct MockDb;

#[async_trait::async_trait]
impl Database for MockDb {
    async fn get_dad_by_id(&self, id: u64) -> Result<Dad, AppError> {
        // pretend async DB call
        let name = format!("MockDad-{}", id);
        Ok(Dad { age: 50 + (id as i32), name, alive: id % 2 == 0 })
    }
}

// A component that depends on Database. We inject the DB via Arc<dyn Database>
struct DadService {
    db: Arc<dyn Database>,
}

impl DadService {
    pub fn new(db: Arc<dyn Database>) -> Self {
        Self { db }
    }

    pub async fn load_and_greet(&self, id: u64) -> Result<(), AppError> {
        let dad = self.db.get_dad_by_id(id).await?;
        dad.greet();
        Ok(())
    }
}

// -----------------------------
// 7) Abstract Factory returning trait objects
// -----------------------------
trait AnimalDad {
    fn speak(&self);
}

struct AliveDad;
struct DeadDad;

impl AnimalDad for AliveDad { fn speak(&self) { println!("I'm alive: Woof!"); } }
impl AnimalDad for DeadDad  { fn speak(&self) { println!("I'm gone: ..."); } }

fn dad_factory(alive: bool) -> Box<dyn AnimalDad> {
    if alive { Box::new(AliveDad) }
    else { Box::new(DeadDad) }
}

// -----------------------------
// 8) Self-referencing using Arc (simple)
// -----------------------------
struct Node {
    name: String,
    parent: Option<Arc<Node>>,
}

impl Node {
    fn new_root(name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.to_string(), parent: None })
    }

    fn new_child(parent: &Arc<Node>, name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.to_string(), parent: Some(parent.clone()) })
    }
}

// -----------------------------
// 9) State machine constructor
// -----------------------------
enum MachineState {
    Init,
    Running { started_at: u64 },
    Stopped { reason: String },
}

struct Machine {
    id: u64,
    state: MachineState,
}

impl Machine {
    // constructor that sets initial state to Init
    fn new(id: u64) -> Self {
        Self { id, state: MachineState::Init }
    }

    fn start(&mut self, ts: u64) { self.state = MachineState::Running { started_at: ts }; }
    fn stop(&mut self, reason: &str) { self.state = MachineState::Stopped { reason: reason.to_string() }; }
}

// -----------------------------
// 10) Demonstration `main` showing how these pieces fit together
// -----------------------------
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // -- Builder pattern
    let dad = DadBuilder::new().age(55).name("David Kamau").alive(false).build()?;
    dad.greet();

    // -- Zero-copy constructor
    let raw_name = String::from("ZeroCopy Kamau");
    let dad_ref = DadRef::new(&raw_name, 44, true)?;
    println!("DadRef name: {}", dad_ref.name);

    // -- Dependency Injection: inject MockDb into DadService
    let db: Arc<dyn Database> = Arc::new(MockDb);
    let svc = DadService::new(db);
    svc.load_and_greet(1).await?; // async call to mocked DB

    // -- Abstract Factory
    let animal = dad_factory(dad.is_alive());
    animal.speak();

    // -- Self-referencing nodes
    let root = Node::new_root("root");
    let child = Node::new_child(&root, "child1");
    println!("child parent present: {}", child.parent.is_some());

    // -- State machine
    let mut machine = Machine::new(100);
    machine.start(123456789);

    // -- Fallible constructor / try_new
    match Dad::try_new(30, "TryKamau") {
        Ok(d) => println!("Try new succeeded: {}", d.name),
        Err(e) => println!("Try new failed: {}", e),
    }

    // -- Async constructor (illustrative)
    let d2 = Dad::init_from_service("RemoteDad")?;
    d2.greet();

    Ok(())
}

/*
Notes / How to adapt to real crates:
- To use async trait Database, add `async-trait = "0.1"` to Cargo.toml and `use async_trait::async_trait;`
- Replace MockDb with a real Postgres implementation using `sqlx::PgPool` and implement get_dad_by_id by querying the DB.
- To integrate with an HTTP server, create an axum handler that depends on Arc<dyn Database> using axum's extractors.
- Use `redis::Client` for Redis-backed initialization inside an async constructor.
- The macro usage here is simplified for readability; in a real project you'd write the macro and invoke with concrete types or write the macro to accept type identifiers.
- Many constructors are fallible and return `Result<Self, Error>` instead of panicking.
*/
