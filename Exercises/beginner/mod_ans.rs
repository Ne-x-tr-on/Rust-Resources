// ========================
// RUST MODULE PRACTICE ANSWERS
// ========================

fn main() {
    println!("Q1: {}", math_utils::add(2, 3));
    // Q2: printer::greet(); // Error: private function
    auth::login(); auth::logout();
    shapes::geometry::area();
    use shapes::geometry::area;
    area();
    println!("PI: {}", converter::PI);
    tools::Tool::use_tool();
    println!("{}", greetings::hello());
    basic::explain();
    outer::middle::inner::shout();
    a::info(); b::info();
    debug_util::debug();
    parent::child::call_super();

    let settings = config::Settings::new(1, "App");
    println!("Version: {}", settings.version);
    maths::basic::add();
    maths::advanced::integrate();
}



mod math_utils {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

mod printer {
    fn greet() {
        println!("Hello (private)");
    }
    // Uncommenting this line in main will cause a compile error
    // pub fn try_greet() { greet(); }
}

mod auth {
    pub fn login() {
        println!("Logged in");
    }
    pub fn logout() {
        println!("Logged out");
    }
}

mod shapes {
    pub mod geometry {
        pub fn area() {
            println!("Area called");
        }
    }
}

mod converter {
    pub const PI: f64 = 3.1415;
}

mod tools {
    pub struct Tool;
    impl Tool {
        pub fn use_tool() {
            println!("Using tool");
        }
    }
}

mod greetings {
    pub fn hello() -> String {
        "Hello from greetings!".to_string()
    }
}

mod basic {
    pub fn explain() {
        println!("Basic module function");
    }
}

mod outer {
    pub mod middle {
        pub mod inner {
            pub fn shout() {
                println!("Shouting from inner!");
            }
        }
    }
}

mod a {
    pub fn info() {
        println!("Module A info");
    }
}

mod b {
    pub fn info() {
        println!("Module B info");
    }
}

mod debug_util {
    pub(crate) fn debug() {
        println!("Debugging internally");
    }
}

mod parent {
    pub fn call_parent() {
        println!("Called from parent");
    }
    pub mod child {
        pub fn call_super() {
            super::call_parent();
        }
    }
}

mod testing {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(add(2, 2), 4);
        }
    }
}

mod config {
    pub struct Settings {
        pub version: u8,
        name: String, // private
    }

    impl Settings {
        pub fn new(version: u8, name: &str) -> Self {
            Self { version, name: name.to_string() }
        }
    }
}

mod maths {
    pub mod basic {
        pub fn add() {
            println!("Basic add");
        }
    }

    pub mod advanced {
        pub fn integrate() {
            println!("Advanced integrate");
        }
    }
}


