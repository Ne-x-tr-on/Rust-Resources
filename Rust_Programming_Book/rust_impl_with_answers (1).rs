fn main() {
    /* Block 1: Questions 1-5 */
    {
        // 1. Create an immutable variable and print its value.
        let x = 5;
        println!("{}", x);

        // 2. Create a mutable variable and change its value.
        let mut y = 10;
        y = 15;

        // 3. Move ownership of a String to a function.
        fn take_ownership(s: String) {
            println!("{}", s);
        }
        let s = String::from("hello");
        take_ownership(s);

        // 4. Use borrowing with & to avoid moving ownership.
        fn borrow(s: &String) {
            println!("{}", s);
        }
        let s2 = String::from("hello");
        borrow(&s2);

        // 5. Use mutable borrowing with &mut to modify a value in a function.
        fn mutate(s: &mut String) {
            s.push_str(" world");
        }
        let mut s3 = String::from("hello");
        mutate(&mut s3);
        println!("{}", s3);
    }

    /* Block 2: Questions 6-10 */
    {
        // 6. Write a function that returns the sum of two integers.
        fn sum(a: i32, b: i32) -> i32 {
            a + b
        }
        println!("Sum: {}", sum(3, 5));

        // 7. Write a function that returns a boolean if a number is even.
        fn is_even(n: i32) -> bool {
            n % 2 == 0
        }
        println!("Is even: {}", is_even(4));

        // 8. Call a function from inside another function.
        fn inner() -> i32 { 5 }
        fn outer() -> i32 { inner() }
        println!("Outer: {}", outer());

        // 9. Use a function that returns a tuple.
        fn get_tuple() -> (i32, f64, char) {
            (1, 2.0, 'a')
        }
        let tup = get_tuple();
        println!("Tuple: {:?}", tup);

        // 10. Define a function that returns nothing (unit type).
        fn do_nothing() {
            // Implicitly returns ()
        }
        do_nothing();
    }

    /* Block 3: Questions 11-15 */
    {
        // 11. Define a struct named `Book` with title and pages.
        struct Book {
            title: String,
            pages: u32,
        }

        // 12. Create an instance of the `Book` struct.
        let book = Book {
            title: String::from("Rust Essentials"),
            pages: 300,
        };

        // 13. Access fields of a struct instance.
        println!("Title: {}, Pages: {}", book.title, book.pages);

        // 14. Use struct update syntax to create a second book.
        let book2 = Book {
            title: String::from("Advanced Rust"),
            ..book
        };
        println!("Title: {}, Pages: {}", book2.title, book2.pages);

        // 15. Create a tuple struct with 3 values.
        struct Point(i32, i32, i32);
        let origin = Point(0, 0, 0);
        println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);
    }

    /* Block 4: Questions 16-20 */
    {
        struct Article {
            content: String,
        }

        // 16. Implement a method `summary` for a struct `Article`.
        impl Article {
            fn summary(&self) -> String {
                format!("{}...", &self.content[0..10])
            }
        }

        // 17. Create an `impl` block with a constructor method `new()`.
        impl Article {
            fn new(content: String) -> Self {
                Article { content }
            }
        }

        // 18. Use &self and &mut self in struct methods.
        impl Article {
            fn read(&self) -> &str {
                &self.content
            }
            
            fn edit(&mut self, new_content: String) {
                self.content = new_content;
            }
        }

        // 19. Create multiple methods inside an impl block.
        impl Article {
            fn word_count(&self) -> usize {
                self.content.split_whitespace().count()
            }
        }

        // 20. Call a method on a struct instance.
        let mut article = Article::new(String::from("Rust is a systems programming language"));
        println!("Summary: {}", article.summary());
        article.edit(String::from("Rust focuses on safety and performance"));
        println!("Content: {}", article.read());
    }

    /* Block 5: Questions 21-25 */
    {
        // 21. Create an enum `Message` with 3 variants.
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }

        // 22. Use a match statement to handle enum values.
        let msg = Message::Move { x: 10, y: 20 };
        match msg {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
        }

        // 23. Use `if let` to match a single enum variant.
        let msg2 = Message::Write(String::from("hello"));
        if let Message::Write(text) = &msg2 {
            println!("Text message: {}", text);
        }

        // 24. Create an enum with data in its variants.
        enum WebEvent {
            PageLoad,
            KeyPress(char),
            Paste(String),
        }

        // 25. Call a method on an enum using an impl block.
        impl WebEvent {
            fn describe(&self) {
                match self {
                    WebEvent::PageLoad => println!("Page loaded"),
                    WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
                    WebEvent::Paste(s) => println!("Pasted text: {}", s),
                }
            }
        }
        let event = WebEvent::KeyPress('a');
        event.describe();
    }

    /* Block 6: Questions 26-30 */
    {
        // 26. Define a trait called `Printable` with one method.
        trait Printable {
            fn print(&self);
        }

        // 27. Implement a trait for a struct.
        struct Report {
            id: u32,
        }

        impl Printable for Report {
            fn print(&self) {
                println!("Report ID: {}", self.id);
            }
        }

        // 28. Use trait bounds in a generic function.
        fn print_item<T: Printable>(item: T) {
            item.print();
        }
        let report = Report { id: 1001 };
        print_item(report);

        // 29. Use `dyn` keyword for dynamic dispatch.
        fn print_dynamic(item: &dyn Printable) {
            item.print();
        }
        let report2 = Report { id: 1002 };
        print_dynamic(&report2);

        // 30. Implement Debug trait manually using `fmt`.
        use std::fmt;
        struct Debuggable(i32);

        impl fmt::Debug for Debuggable {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Debuggable({})", self.0)
            }
        }
        println!("{:?}", Debuggable(42));
    }

    /* Block 7: Questions 31-35 */
    {
        // 31. Show an example where ownership is moved and causes error.
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}", s1); // This would cause an error

        // 32. Fix ownership issues using borrowing.
        let s3 = String::from("world");
        let s4 = &s3;
        println!("{}", s3); // Fixed by using reference

        // 33. Use clone() to copy a String.
        let s5 = String::from("rust");
        let s6 = s5.clone();
        println!("{} {}", s5, s6);

        // 34. Demonstrate dangling reference and fix it.
        // fn dangle() -> &String {
        //     let s = String::from("dangle");
        //     &s
        // } // s goes out of scope, reference would dangle

        // Fixed version:
        fn no_dangle() -> String {
            let s = String::from("safe");
            s
        }
        let _s7 = no_dangle();

        // 35. Pass ownership to a function and return it back.
        fn take_and_return(s: String) -> String {
            println!("Received: {}", s);
            s
        }
        let s8 = String::from("borrow");
        let s9 = take_and_return(s8);
        println!("Returned: {}", s9);
    }

    /* REPEAT PATTERN FOR PRACTICE QUESTIONS */
    // Blocks 8-14: Questions 36-70 (repeat of 1-35)
    {
        /* Block 8: Questions 36-40 (repeat of 1-5) */
        {
            let x = 10;
            println!("{}", x);
            
            let mut y = 20;
            y = 25;
            
            fn take_ownership2(s: String) {
                println!("{}", s);
            }
            let s = String::from("hello2");
            take_ownership2(s);
            
            fn borrow2(s: &String) {
                println!("{}", s);
            }
            let s2 = String::from("hello2");
            borrow2(&s2);
            
            fn mutate2(s: &mut String) {
                s.push_str(" world");
            }
            let mut s3 = String::from("hello2");
            mutate2(&mut s3);
            println!("{}", s3);
        }

        /* Block 9: Questions 41-45 (repeat of 6-10) */
        {
            fn sum2(a: i32, b: i32) -> i32 { a + b }
            println!("Sum2: {}", sum2(4, 6));
            
            fn is_even2(n: i32) -> bool { n % 2 == 0 }
            println!("Is even2: {}", is_even2(5));
            
            fn inner2() -> i32 { 10 }
            fn outer2() -> i32 { inner2() }
            println!("Outer2: {}", outer2());
            
            fn get_tuple2() -> (i32, f64, char) { (2, 3.0, 'b') }
            let tup = get_tuple2();
            println!("Tuple2: {:?}", tup);
            
            fn do_nothing2() {}
            do_nothing2();
        }

        /* Block 10: Questions 46-50 (repeat of 11-15) */
        {
            struct Book2 {
                title: String,
                pages: u32,
            }
            
            let book = Book2 {
                title: String::from("Rust Cookbook"),
                pages: 400,
            };
            
            println!("Title2: {}, Pages: {}", book.title, book.pages);
            
            let book2 = Book2 {
                title: String::from("Rust Patterns"),
                ..book
            };
            println!("Title2: {}, Pages: {}", book2.title, book2.pages);
            
            struct Color(u8, u8, u8);
            let red = Color(255, 0, 0);
            println!("Color: ({}, {}, {})", red.0, red.1, red.2);
        }

        /* Block 11: Questions 51-55 (repeat of 16-20) */
        {
            struct Document {
                content: String,
            }
            
            impl Document {
                fn summary(&self) -> String {
                    format!("{}...", &self.content[0..15])
                }
                
                fn new(content: String) -> Self {
                    Document { content }
                }
                
                fn content(&self) -> &str {
                    &self.content
                }
                
                fn edit(&mut self, new_content: String) {
                    self.content = new_content;
                }
                
                fn word_count(&self) -> usize {
                    self.content.split_whitespace().count()
                }
            }
            
            let mut doc = Document::new(String::from("The Rust programming language"));
            println!("Doc Summary: {}", doc.summary());
            doc.edit(String::from("Rust enforces memory safety"));
            println!("Content: {}", doc.content());
        }

        /* Block 12: Questions 56-60 (repeat of 21-25) */
        {
            enum Command {
                Exit,
                MoveTo(i32, i32),
                Text(String),
            }
            
            let cmd = Command::MoveTo(5, 10);
            match cmd {
                Command::Exit => println!("Exit"),
                Command::MoveTo(x, y) => println!("Move to ({}, {})", x, y),
                Command::Text(t) => println!("Text: {}", t),
            }
            
            let cmd2 = Command::Text(String::from("goodbye"));
            if let Command::Text(t) = &cmd2 {
                println!("Text command: {}", t);
            }
            
            enum AppEvent {
                Load,
                Input(String),
            }
            
            impl AppEvent {
                fn log(&self) {
                    match self {
                        AppEvent::Load => println!("App loaded"),
                        AppEvent::Input(s) => println!("Input: {}", s),
                    }
                }
            }
            let event = AppEvent::Input(String::from("login"));
            event.log();
        }

        /* Block 13: Questions 61-65 (repeat of 26-30) */
        {
            trait Loggable {
                fn log(&self);
            }
            
            struct Event {
                id: u32,
            }
            
            impl Loggable for Event {
                fn log(&self) {
                    println!("Event ID: {}", self.id);
                }
            }
            
            fn log_item<T: Loggable>(item: T) {
                item.log();
            }
            let event = Event { id: 2001 };
            log_item(event);
            
            fn log_dynamic(item: &dyn Loggable) {
                item.log();
            }
            let event2 = Event { id: 2002 };
            log_dynamic(&event2);
            
            use std::fmt;
            struct Debuggable2(i32);
            
            impl fmt::Debug for Debuggable2 {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Debuggable2({})", self.0)
                }
            }
            println!("{:?}", Debuggable2(99));
        }

        /* Block 14: Questions 66-70 (repeat of 31-35) */
        {
            let s1 = String::from("test");
            let s2 = s1;
            // println!("{}", s1); // Error
            
            let s3 = String::from("reference");
            let s4 = &s3;
            println!("{}", s3);
            
            let s5 = String::from("clone");
            let s6 = s5.clone();
            println!("{} {}", s5, s6);
            
            fn no_dangle2() -> String {
                String::from("no dangle")
            }
            let _s7 = no_dangle2();
            
            fn take_and_return2(s: String) -> String {
                println!("Received: {}", s);
                s
            }
            let s8 = String::from("ownership");
            let s9 = take_and_return2(s8);
            println!("Returned: {}", s9);
        }
    }

    /* FINAL PRACTICE BLOCKS */
    // Blocks 15-20: Questions 71-100 (repeat of 1-30)
    {
        /* Block 15: Questions 71-75 (repeat of 1-5) */
        {
            let x = 15;
            println!("{}", x);
            
            let mut y = 30;
            y = 35;
            
            fn take_ownership3(s: String) {
                println!("{}", s);
            }
            let s = String::from("hello3");
            take_ownership3(s);
            
            fn borrow3(s: &String) {
                println!("{}", s);
            }
            let s2 = String::from("hello3");
            borrow3(&s2);
            
            fn mutate3(s: &mut String) {
                s.push_str(" world");
            }
            let mut s3 = String::from("hello3");
            mutate3(&mut s3);
            println!("{}", s3);
        }

        /* Block 16: Questions 76-80 (repeat of 6-10) */
        {
            fn sum3(a: i32, b: i32) -> i32 { a + b }
            println!("Sum3: {}", sum3(5, 7));
            
            fn is_even3(n: i32) -> bool { n % 2 == 0 }
            println!("Is even3: {}", is_even3(6));
            
            fn inner3() -> i32 { 15 }
            fn outer3() -> i32 { inner3() }
            println!("Outer3: {}", outer3());
            
            fn get_tuple3() -> (i32, f64, char) { (3, 4.0, 'c') }
            let tup = get_tuple3();
            println!("Tuple3: {:?}", tup);
            
            fn do_nothing3() {}
            do_nothing3();
        }

        /* Block 17: Questions 81-85 (repeat of 11-15) */
        {
            struct Book3 {
                title: String,
                pages: u32,
            }
            
            let book = Book3 {
                title: String::from("Rust in Action"),
                pages: 500,
            };
            
            println!("Title3: {}, Pages: {}", book.title, book.pages);
            
            let book2 = Book3 {
                title: String::from("Rust for Rustaceans"),
                ..book
            };
            println!("Title3: {}, Pages: {}", book2.title, book2.pages);
            
            struct RGB(u8, u8, u8);
            let blue = RGB(0, 0, 255);
            println!("RGB: ({}, {}, {})", blue.0, blue.1, blue.2);
        }

        /* Block 18: Questions 86-90 (repeat of 16-20) */
        {
            struct Paper {
                content: String,
            }
            
            impl Paper {
                fn summary(&self) -> String {
                    format!("{}...", &self.content[0..20])
                }
                
                fn new(content: String) -> Self {
                    Paper { content }
                }
                
                fn content(&self) -> &str {
                    &self.content
                }
                
                fn edit(&mut self, new_content: String) {
                    self.content = new_content;
                }
                
                fn word_count(&self) -> usize {
                    self.content.split_whitespace().count()
                }
            }
            
            let mut paper = Paper::new(String::from("This is a research paper about Rust"));
            println!("Paper Summary: {}", paper.summary());
            paper.edit(String::from("Rust's ownership system prevents data races"));
            println!("Content: {}", paper.content());
        }

        /* Block 19: Questions 91-95 (repeat of 21-25) */
        {
            enum Action {
                Stop,
                Go(i32, i32),
                Message(String),
            }
            
            let action = Action::Go(7, 14);
            match action {
                Action::Stop => println!("Stop"),
                Action::Go(x, y) => println!("Go to ({}, {})", x, y),
                Action::Message(msg) => println!("Message: {}", msg),
            }
            
            let action2 = Action::Message(String::from("hello world"));
            if let Action::Message(msg) = &action2 {
                println!("Action message: {}", msg);
            }
            
            enum GameEvent {
                Start,
                PlayerInput(char),
            }
            
            impl GameEvent {
                fn describe(&self) {
                    match self {
                        GameEvent::Start => println!("Game started"),
                        GameEvent::PlayerInput(c) => println!("Player input: {}", c),
                    }
                }
            }
            let event = GameEvent::PlayerInput('x');
            event.describe();
        }

        /* Block 20: Questions 96-100 (repeat of 26-30) */
        {
            trait Displayable {
                fn display(&self);
            }
            
            struct Record {
                id: u32,
            }
            
            impl Displayable for Record {
                fn display(&self) {
                    println!("Record ID: {}", self.id);
                }
            }
            
            fn display_item<T: Displayable>(item: T) {
                item.display();
            }
            let record = Record { id: 3001 };
            display_item(record);
            
            fn display_dynamic(item: &dyn Displayable) {
                item.display();
            }
            let record2 = Record { id: 3002 };
            display_dynamic(&record2);
            
            use std::fmt;
            struct Debuggable3(i32);
            
            impl fmt::Debug for Debuggable3 {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Debuggable3({})", self.0)
                }
            }
            println!("{:?}", Debuggable3(77));
        }
    }
}