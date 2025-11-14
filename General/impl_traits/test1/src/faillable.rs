#[derive(Debug)]
enum DadError {
    InvalidAge,
    EmptyName,
}

impl Dad {
    fn try_new(age: i32, name: &str, alive: bool) -> Result<Self, DadError> {
        if age <= 0 {
            return Err(DadError::InvalidAge);
        }
        if name.is_empty() {
            return Err(DadError::EmptyName);
        }

        Ok(Self {
            age,
            name: name.to_string(),
            alive,
        })
    }
}

match Dad::try_new(55, "Kamau", true) {
    Ok(d) => println!("Dad created: {:?}", d),
    Err(e) => println!("Failed: {:?}", e),
}
