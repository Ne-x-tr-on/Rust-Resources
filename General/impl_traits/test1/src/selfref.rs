use std::sync::Arc;

struct Dad {
    name: String,
    parent: Option<Arc<Dad>>,
}

impl Dad {
    fn new_root(name: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            parent: None,
        })
    }
}


let root = Dad::new_root("Kamau");
