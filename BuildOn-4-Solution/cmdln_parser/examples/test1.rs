/////////////////////////////////////////////////////////////
// cmdln_parser::test1.rs - Command line parser test       //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 13 Feb 2021  //
/////////////////////////////////////////////////////////////

use cmdln_parser::{CmdParser, show_args, show_parse};

fn show_key(cp: & CmdParser, key: &str) {
    if cp.contains_key(key) {
        print!("\n  contains key {:?}", key);
        print!("\n  its value is {:?}", cp.get(&key).unwrap());
    }
    else {
        print!("\n  does not contain key {:?}", key);
    }
}
fn set_defaults(cp:&mut CmdParser) {
    cp.set_default("P", ".");
    cp.set_default("p", "rs");
    cp.set_default("T", " ");
    cp.set_default("H", "true");
    cp.set_default("v", "true");
    cp.set_default("s", "true");
}

fn main() {
    print!("\n  -- basic test of CmdParser --\n");

    let mut cp = CmdParser::new();

    print!("\n  -- cmdln parse --\n");
    cp.parse();
    print!("\n{:#?}", cp);
    println!();

    print!("\n  -- defaults set --\n");
    set_defaults(&mut cp);
    print!("\n{:#?}", cp);
    println!();

    cp.set_default("foo", "bar");
    cp.empty_values_to_true();

    print!("{}", show_args(&cp));
    println!();

    print!("{}", show_parse(&cp));
    println!();

    show_key(&cp, "arg1");
    show_key(&cp,"noExist");

    cp.insert("yo", "ho");
    print!("\n  {}", show_parse(&cp));

    cp.insert("yo", "hi");
    print!("\n  {}", show_parse(&cp));

    cp.replace_value("yo", "ha");
    print!("\n  {}", show_parse(&cp));

    print!("\n\n  That's all Folks!\n\n");
}