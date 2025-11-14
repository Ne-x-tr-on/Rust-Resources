trait DadTrait {
    fn speak(&self);
}

struct AliveDad;
struct DeadDad;

fn create_dad(alive: bool) -> Box<dyn DadTrait> {
    if alive {
        Box::new(AliveDad)
    } else {
        Box::new(DeadDad)
    }
}

