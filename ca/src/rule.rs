use std::{collections::HashMap, ops::{Add, Mul}};

use crate::{grid::apoint::APoint, state::State};
use crate::errors::SumError;

/// Function that maps one state to another based on
/// its implementation
pub struct RuleFunc<T>(fn(State<T>) -> State<T>);

/// A Rule must be able to use a function to map one state to another
pub trait Rule<T> {
    fn apply(&self, func: RuleFunc<State<T>>) -> State<T>;
}

/// Mapping of each state to the function that determines the state of cell
/// at next step
pub struct Rules<T, U>
    where
    U: Fn(T) -> T {
        rules: HashMap<T, U>,
    }

impl<T, U> Rules<T, U>
where
    U: Fn(T) -> T {

    /// Return a new, empty `Rules` struct
    ///
    /// # Examples
    ///
    /// ```
    /// use ca::rule::*;
    /// let rules: Rules<i32, fn(i32) -> i32> = Rules::new();
    ///
    /// ```
    pub fn new() -> Rules<T, U> {
        Rules {
            rules: HashMap::new(),
        }
    }

    /// Set rules map
    ///
    /// # Examples
    ///
    /// ```
    /// use ca::rule::*;
    /// let mut rules: Rules<i32, fn(i32) ->i32> = Rules::new();
    /// let r = create_rules("RULES");
    /// rules.set_rules(r);
    ///
    /// ```
    pub fn set_rules(&mut self, rule_set: HashMap<T, U>) {
        self.rules = rule_set;
    }
}

// TODO
// pub fn create_rules<T, U>(rules: &str) -> HashMap<T, U> {
//     let mut states: T;
//     let mut transisitions: Vec<U> = Vec::new();
//
//     let mut rules = HashMap::new();
//     rules
// }


pub trait Sum<T> {
    fn sum(&self) -> Result<T, SumError>;
}

impl<'a, T> Sum<i32> for APoint<'a, T, State<'a, i32>>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>, {
    fn sum(&self) -> Result<i32, SumError> {
        let mut s = 0;
        for n in self.neighbours(){
            let v = n.cur_state();
            s += match v {
                Some(i) => *i.val().val(),
                None => return Err(SumError),
            }
        }
        Ok(s)
    }
}
