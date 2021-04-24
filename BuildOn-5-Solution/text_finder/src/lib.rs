///////////////////////////////////////////////////////////////
// text_finder::lib.rs - search dir tree for files with text //
// ver 1.0                                                   //
// Jim Fawcett, https://JimFawcett.github.io, 22 Feb 2021    //
///////////////////////////////////////////////////////////////

#![allow(dead_code)]

/*-- dependencies --*/
use dir_nav::{DirNav, replace_sep};
use text_search::{Finder};
use display::{GenOut};
use cmdln_parser::{CmdParser};
use std::path::{Path, PathBuf};
use std::collections::hash_map::{Entry};

/*---------------------------------------------------------
  Executive type
  - builds processing pipeline
  - applies command line options
  - starts directory navigation
*/
#[derive(Debug, Default)]
pub struct Executive {
    cp: CmdParser,
    dn: DirNav<Finder<GenOut>>,
}
impl Executive {
    pub fn new() -> Executive {
        Executive {
            /* wraps HashMap of options */
            cp: CmdParser::new(),
            /* directory navigator bound to Finder */
            dn: DirNav::<Finder<GenOut>>::new(),
        }
    }
    /*-- parse command line, apply options --*/
    pub fn process_cmdln(&mut self) -> bool {

        self.cp.parse();
        self.cp.empty_values_to_true(); /* /opt => /opt true */

        /* help message */
        if self.get_first("h") == "true" {
            print!("\n{}\n\n", self.help_msg());
            return false;
        }
        /* set default values if not specified */
        self.set_defaults();
    
        let _attrib = self.cp.get_parse();
        
        /* verbose option */
        if self.get_first("v") == "true" {
            print!("\n  Command line:\n  ");
            let iter = std::env::args();
            for arg in iter.skip(1) {
                print!("{} ", arg);
            }
            // print!("\n  Parsed attributes:");
            // for arg in _attrib {
            //     print!("\n    {:?}", arg);
            // }
            println!();
        }
    
        /* apply patterns */
        if let Some(pats) = self.cp.get("p") {
            for pat in pats {
                let p = Path::new(pat);
                self.dn.add_patt(p);
            }
        }
        
        /* set search text in Finder */
        if let Some(txts) = self.cp.get("T") {
            if !txts.is_empty() {
                self.dn.get_app().set_txt(&txts[0]);
            }
        }

        if let Some(recur) = self.cp.get("s") {
            if !recur.is_empty() && recur[0].as_str() == "false" {
                self.dn.recurse(false);
            }
        }
    
        /* get GenOut */
        let out = self.dn.get_app().get_app();
    
        /* apply Hidden attribute */
        if let Some(hides) = self.cp.get("H") {
            if !hides.is_empty() {
                out.set_hide_unmatched(hides[0].as_str() == "true");
                // out.set_hide_unmatched(hides[0] == *"true");
            }
        }
    
        /* apply show All attribute, both matched and unmatched */
        if let Some(outs) = self.cp.get("A") {
            if !outs.is_empty() {
                out.show_all(outs[0].as_str() == "true");
            }
        }
    
        /* apply Debug ouput attribute */
        if let Some(outs) = self.cp.get("D") {
            if !outs.is_empty() {
                let do_debug: bool = outs[0].as_str() == "true";
                out.set_debug(do_debug);
                if do_debug {
                    /* show Finder instance */
                    print!("\n{:#?}\n", self.dn.get_app());
                }
            }
        }
        true
    }
    pub fn get_dirs(&self) -> usize {
        self.dn.get_dirs()
    }
    pub fn get_files(&self) -> usize {
        self.dn.get_files()
    }
    pub fn get_first(&self, key: &str) -> String {
        if let Some(vals) = self.cp.get(key) {
            if !vals.is_empty() {
                return vals[0].clone();
            }
        }
        "".to_string()
    }
    /*-- return reference to CmdParser --*/
    pub fn get_cparser(&self) -> &CmdParser {
        &self.cp
    }
    /*-- return reference to CmdParser --*/
    pub fn get_cparser_mut(&mut self) -> &mut CmdParser {
        &mut self.cp
    }
    /*---------------------------------------------------------
      Add attribute-value pair: example p-h.  Works for both
      cases:
      - attribute does not exist
      - attribute exists and has vector of values
    */
    pub fn set_attribute_item(&mut self, attr: &str, val: &str) {
        let vs = val.to_string();
        if attr == "p" {
            let hm = &mut self.cp;
            if hm.contains_key("p") {
                if let Some(v) = hm.get_mut("p") {
                    if !v.contains(&vs) {
                        self.dn.add_patt(&Path::new(val));
                        v.push(vs);
                        // print!("\n  added item {:?}", (attr, val));
                    }
                }
            }
            else {
                hm.insert(attr, val);
                self.dn.add_patt(&Path::new(val));
                // print!("\n  added item {:?}", (attr, val));
            }
        }
        else {
            let hm = self.cp.get_parse_mut();
            let entry = hm.entry(attr.to_string());
            if let Entry::Occupied(mut rentry) = entry {
                let v: &mut Vec<String> = rentry.get_mut();
                if v.is_empty() {
                    v.push(vs);
                    // print!("\n  added item {:?}", (attr, val));
                }
                else if v[0].as_str() == "true" 
                     || v[0].as_str() == "false" {
                    v[0] = vs;
                    // print!("\n  added item {:?}", (attr, val));
                }
                else {
                    v.push(vs);
                    // print!("\n  added item {:?}", (attr, val));
                }
            }
            else {
                hm.insert(attr.to_string(), vec![vs]);
                // print!("\n  added item {:?}", (attr, val));
            }
        }
    } 
    /*---------------------------------------------------------
      Set default for attribute-value pair.  Does nothing if
      attribute already exists.
    */
    pub fn set_default(&mut self, attr: &str, val: &str) {
        self.cp.set_default(attr, val);
        if attr == "p" {
                self.dn.add_patt(Path::new(val));
        }
        if let Some(vec) = self.cp.get("v") {
            if vec[0].contains("true") {
                print!("\n  added (key, val) = {:?}", (attr, val));
            }
        }
    }
    /*---------------------------------------------------------
      Called after parsing command line, won't replace existing. 
      Client may add additional default with set_default, or
      set_attribute_item.
    */
    fn set_defaults(&mut self) {
        self.cp.set_default("P", ".");       // Path
        self.cp.set_default("s", "true");    // recurse
        self.cp.set_default("H", "true");    // hide unmatched
        self.cp.set_default("v", "true");    // verbose
        self.cp.set_default("T", " ");       // Text to find
        self.cp.set_default("D", "false");   // Debug out
        self.cp.set_default("A", "false");
    }
    /*-- call if key "h" exists, then return --*/
    pub fn help_msg(&self) -> String {
        let mut hm = String::new();
        hm.push_str("Sample Command Line Format:");
        hm.push_str("\n  /P path /p rs /p h cpp /T text_to_find");
        hm.push_str("\n\nAttributes:");
        hm.push_str("\n  /P .         => start path is \".\"");
        hm.push_str("\n  /p rs h cpp  => patterns are \"rs\", \"h\", \"cpp\"");
        hm.push_str("\n  /T abc       => search text is \"abc\"");
        hm.push_str("\n  /s true      => recursive search");
        hm.push_str("\n  /H true      => hide dirs with no matches");
        hm.push_str("\n  /v true      => verbose - show parse");
        hm.push_str("\n  /A true      => show all matching files");
        hm.push_str("\n  /D true      => show debug output");
        hm
    }
    /* convert relative Path to absolute PathBuf */
    #[allow(non_snake_case)]
    pub fn to_abs_path(&self, path: &Path) -> PathBuf {
        let rslt = std::fs::canonicalize(path);
        if let Ok(abs_path) = rslt {
            let abs_OsString = replace_sep(&abs_path);
            let rslt = abs_OsString.into_string();
            if let Ok(abs_string) = rslt {
                let tail_abs: String = abs_string.chars().skip(4).collect();
                return PathBuf::from(tail_abs);
            }
        }
        PathBuf::from("")
    }
    /* start DirNav at specified path */
    pub fn start(&mut self) -> bool {
        if let Ok(curr_dir) = std::env::current_dir() {
            print!("\n  current directory:\n  {:?}", replace_sep(&curr_dir));
        }
        if let Some(paths) = self.cp.get("P") {
            let mut path_string = String::new();
            if !paths.is_empty() {
                path_string = paths[0].clone();
            }
            let start_path = Path::new(&path_string);
            print!("\n  start path:\n  {:?}\n", self.to_abs_path(start_path));
            let rslt = self.dn.visit(start_path);
            println!();
            match rslt {
                Ok(()) => true,
                Err(_) => false,
            }
        }
        else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construction() {
        let mut ex = Executive::new();
        /*-- have functional CmdParser? --*/
        ex.set_defaults();
        assert_eq!(ex.get_first("P"), ".".to_string());
        /*-- have functional DirNav<Finder>? --*/
        let patt = Path::new("foo");
        ex.dn.add_patt(patt);
        let patts = ex.dn.get_patts();
        assert_eq!(patt, patts[0]);
    }
    /*-------------------------------------------
      Have to test rest with test mains
      - too many moving parts for simple unit tests
    */
}
