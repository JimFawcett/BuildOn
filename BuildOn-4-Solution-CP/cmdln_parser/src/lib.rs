/////////////////////////////////////////////////////////////
// cmdln_parser::lib.rs - Command line parser              //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 13 Feb 2021  //
/////////////////////////////////////////////////////////////

#![allow(dead_code)]

use std::collections::*;
use std::collections::hash_map::Entry;

/*-----------------------------------------------
  Given command line: /Path . /p .rs /p .h .cpp
  
  CmdParser::parse() generates:
  - args: Vec<String> with each of the arguments, above
  - attrib: {
      "Path".to_string(): vec![".".to_string()]
      "p".to_string(): 
      vec![
        ".rs".to_string(), 
        ".h".to_string(), 
        ".cpp".to_string()
      ]
    }
*/
#[derive(Debug, Default)]
pub struct CmdParser {
    /*-- key, value pairs, e.g., /P vec!["."] --*/
    attrib: HashMap<String, Vec<String>>,
    /*-- command line arguments, e.g., /P . /p .rs --*/
    args: Vec<String>
}
impl CmdParser {
    pub fn new() -> CmdParser {
        CmdParser {
            attrib: HashMap::<String, Vec<String>>::new(),
            args : Vec::<String>::new()
        }
    }
    /*-- pass test arguments to parser --*/
    pub fn set_args(&mut self, a: Vec<String>) {
        self.args = a;
    }
    /*-- collect command line arguments into HashMap --*/
    pub fn parse(&mut self) {
        if self.args.is_empty() {
            self.args = std::env::args().collect();
        }
        /*-- attr is current attribute, e.g., /x --*/
        let mut attr = String::new();
        for item in &self.args {
            let mut iter = item.chars();
            /*-- ch is the first char of item --*/
            if let Some(ch) = iter.next() {
                if ch == '/' {
                    /*-- collect rest of chars --*/
                    attr = iter.collect();
                    /*-- if entry has no value make it Vec<String> --*/
                    self.attrib.entry(attr.clone())
                               .or_insert_with(|| { Vec::<String>::new() });
                }
                /*-- if collected at least one attribute --*/
                else if !attr.is_empty() {
                    let value = item.to_string();
                    self.attrib.entry(attr.clone())
                                .and_modify(|v| v.push(value));
                }
            }
        }
    }
    pub fn get(&self, key : &str) -> Option<&Vec<String>> {
        self.attrib.get(key)
    }
    pub fn get_mut(&mut self, key : &str) -> Option<&mut Vec<String>> {
        self.attrib.get_mut(key)
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.attrib.contains_key(key)
    }
    pub fn get_parse(&self) -> &HashMap<String, Vec<String>> {
        &self.attrib
    }
    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
    /*-------------------------------------------
      set default values after invoking parse().
      If the command line contains "/key value" 
      we don't want to change its value(s).
    */
    pub fn set_default(&mut self, key: &str, value: &str) {
        /*-- no entry if key already exists --*/
        if self.attrib.contains_key(key) {
            /* return; */
        }
        else {
            self.attrib.insert(
                key.to_string(), 
                vec![value.to_string()]
            );
        }
    }
    /*-- if any values are empty Vecs push "true".to_string() --*/
    pub fn empty_values_to_true(&mut self) {
        for value in self.attrib.values_mut() {
            if value.is_empty() {
                value.push("true".to_string());
            }
        }
    }
    /*-----------------------------------------------------
      - if key does not exist, create Vec<String> and push v,
        then insert into attrib.
      - if the key exists, then v is pushed onto that item's
        value vector.
    */
    pub fn insert(&mut self, key: &str, v: &str) {
        let ks = key.to_string();
        let vs = v.to_string();

        let e = self.attrib.entry(ks.clone());
        if let Entry::Occupied(mut o) = e {
            o.get_mut().push(vs);
        }
        else {
            let v = vec![vs];
            self.attrib.insert(ks, v);
        }
    }
    /*-----------------------------------------------------
      - if key exists, replace its value with vec![val]
      - otherwise, insert(key, vec![val])
    */
    pub fn replace_value(&mut self, key: &str, val: &str) {
        let ks = key.to_string();
        let vs = val.to_string();

        let e = self.attrib.entry(ks.clone());
        if let Entry::Occupied(mut o) = e {
            o.get_mut().clear();
            o.get_mut().push(vs);
        }
        else {
            let v = vec![vs];
            self.attrib.insert(ks, v);
        }
    }
}

pub fn show_args(cp: &CmdParser) -> String {
    let mut temp = "\n  cmdln arguments:".to_string();
    let iter = cp.get_args().iter();
    let prefix = "\n    ";
    for item in iter {
        temp.push_str(prefix);
        temp.push_str(item.as_str());
    }
    temp
}

pub fn show_parse(cp: &CmdParser) -> String {
    let prefix = "\n    ";
    let mut temp = "\n  cmdln parse:".to_string();
    let iter = cp.get_parse().iter();
    for item in iter {
        temp.push_str(prefix);
        let entry = format!("{:?}", item);
        temp.push_str(entry.as_str());
    }
    temp
}

/*-- unit tests --*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construction() {
        let cp = CmdParser::new();
        let v = Vec::<String>::new();
        assert_eq!(cp.args, v);
        let h = HashMap::<String, Vec<String>>::new();
        assert_eq!(cp.attrib, h);
    }
    #[test]
    fn parse() {
        let args = vec![
            "/arg1".to_string(), 
            "ab".to_string(), 
            "/arg2".to_string(), 
            "cd".to_string(), 
            "ef".to_string()];
        let mut cp = CmdParser::new();
        cp.set_args(args);
        cp.parse();
        assert_eq!(cp.get_parse()["arg1"], vec!["ab"]);
        assert_eq!(cp.get_parse()["arg2"], vec!["cd", "ef"]);
    }
}
