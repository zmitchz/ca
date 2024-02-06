/// A state in an automotan
pub struct State<'a, T>{
    name: &'a str,
    val: T
}

impl<'a, T> State<'a, T>  {
    /// Return a new, named state with a value
    pub fn new(name: &'a str, val: T) -> State<'a, T> {
        State {
            name,
            val,
        }
    }

    /// Get state value
    pub fn val(&self) -> &T {
        &self.val
    }

    /// Get state name
    pub fn name(&self) -> &str {
        self.name
    }
}

