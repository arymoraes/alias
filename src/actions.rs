pub enum Action {
    Run,
    Add,
}

impl Action {
    pub fn from_str(action: &str) -> Self {
        match action {
            "run" => Action::Run,
            "add" => Action::Add,
            &_ => panic!("Invalid action"),
        }
    }
}
