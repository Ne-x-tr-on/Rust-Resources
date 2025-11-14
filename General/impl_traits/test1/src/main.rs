struct Dad {
    age: i32,
    name: String,
    alive: bool,
}
impl Dad {
    fn greet(&self) {
        println!("Hello Dad");
    }
    fn status(&self) {
        if self.alive == true {
            println!("Hey  just started on Opticlass the Software we were chatting about");
        } else if self.alive == false {
            println!("You left a legacy its working even if you are away");
        }
    }
    fn tellage(&self) {
        println!("{} age was {}", self.name, self.age);
    }
    fn revive(&mut self) {
        self.alive = true;
        println!("{}", self.alive);
    }
    fn new(age: i32, name: &str, alive: bool) -> Self {
        Self {
            age,
            name: name.to_string(),
            alive,
        }
    }
}

trait Legacy {
    fn legacy_message(&self);
}

impl Legacy for Dad {
    fn legacy_message(&self) {
        if self.alive {
            println!("Your Legacy is still in progress!");
        } else {
            println!("Your Legacy lives on");
        }
    }
}

fn count_alive(dads:&Vec<Dad>)->usize{
    dads.iter().filter(|d|d.alive).count()
}

fn main() {
    let mut d = Dad {
        age: 55,
        name: "David Kamau".to_string(),
        alive: false,
        // alive: false,
    };
    d.status();
    d.tellage();
    d.revive();

    let dads = vec![
        Dad::new(55, "David Kamau", false),
        Dad::new(45, "Newton", true),
        Dad::new(50, "David Kamau", true),
    ];

    for d in &dads{
        d.status();
    };

    let alive_count = count_alive(&dads);
    println!("{} Dads are alive",alive_count);
   

}


