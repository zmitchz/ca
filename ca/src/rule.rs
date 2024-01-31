use std::collections::HashMap;

/// Mapping of each state to the function that determines the state of cell 
/// at next step
pub struct Rules<T, U>
    where
        T: Fn(T) -> U, {
    rules: HashMap<T, U>,
}

impl<T, U> Rules<T, U>
    where
        T: Fn(T) -> U, {
    pub fn new() -> Rules<T, U> {
        Rules {
            rules: HashMap::new(),
        }
    }

    pub fn set_rules(&mut self, rule_set: HashMap<T, U>) {
        self.rules = rule_set;
    }
}

pub fn create_rules<T, U>(rules: &str) -> HashMap<T, U> {
    let mut states: T;
    let mut transisitions: Vec<U> = Vec::new();

    let mut rules = HashMap::new();
    rules
}

