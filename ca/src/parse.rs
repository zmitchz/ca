use regex::Regex;

use crate::grid::neighbourhood::NType;

pub struct ParsedCA<'a>{
    name: Option<&'a str>,
    neighbourhoods: Vec<NType>,
    states: Option<&'a str>,
    string: Option<String>,
    transitions: Option<&'a str>,
}

impl<'a> ParsedCA<'a> {
    pub fn new() -> Self { 
        Self { 
            string: None,
            name: None,
            neighbourhoods: Vec::new(),
            states: None,
            transitions: None,
        }
    }


    pub fn name(&self) -> Option<&str> { self.name }

    pub fn neighbourhoods(&self) -> &[NType] { self.neighbourhoods.as_ref() }

    pub fn states(&self) -> Option<&str> { self.states }

    pub fn string(&self) -> Option<&String> { self.string.as_ref() }

    pub fn transitions(&self) -> Option<&str> { self.transitions }

    pub fn set_name(&mut self, name: Option<&'a str>) { self.name = name; }

    pub fn set_neighbourhoods(&mut self, neighbourhoods: Vec<NType>) { self.neighbourhoods = neighbourhoods; }

    pub fn set_states(&mut self, states: Option<&'a str>) { self.states = states; }

    pub fn set_string(&mut self, string: Option<String>) { self.string = string; }

    pub fn set_transitions(&mut self, transitions: Option<&'a str>) { self.transitions = transitions; }
}

pub fn parse_ca_string<'a>(string: &str) -> ParsedCA<'a> {
    let mut pca = ParsedCA::new();
    pca.set_string(Some(string.to_string()));
    pca

}

impl<'a> From<String> for ParsedCA<'a> {
    fn from(value: String) -> Self {
        parse_ca_string(&value)
    }
}
