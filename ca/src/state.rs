/// A state in an automotan

pub struct State<'a, T>{
    name: &'a str,
    val: T
}

impl<'a, T> State<'a, T>  {
    pub fn new(name: &'a str, val: T) -> State<'a, T> {
        State {
            name,
            val,
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn set_val(&mut self, new_val: T) {
        self.val = new_val;
    }

}

