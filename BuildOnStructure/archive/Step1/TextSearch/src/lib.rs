///////////////////////////////////////////////////////////
// BuildOnStructure::TextSearch.rs                       //
//   - demonstrate code structure for Step #1            //
// Jim Fawcett, https://JimFawcett.github.io, 14 Jan 21  //
///////////////////////////////////////////////////////////

/*-----------------------------------------------
  Trait is not needed here, but will be in Step #4
*/
pub trait ToOut {
    fn set_dir(&mut self, dir: &str);
    fn set_file(&mut self, rslt: (&str, bool, &str));
}
/*-----------------------------------------------
  GenOut converts Finder's data into information
  for user.
*/
#[derive(Debug)]
pub struct GenOut {
    dir: String
}
/*-- implement trait --------------------------*/
impl ToOut for GenOut {
    fn set_dir(&mut self, rdir: &str) {
        self.dir = rdir.to_string();
        print!("\n  {}", rdir);
    }
    fn set_file(&mut self, rslt: (&str, bool, &str)) {
        let (file, found, text) = rslt;
        if found {
            print!("\n    {}: {:?} found", file, text);
        }
        else {
            print!("\n    {}: {:?} not found", file, text);
        }
    }
}
/*-- implement other member functions ---------*/
impl GenOut {
    pub fn new() -> GenOut {
        GenOut {
            dir: String::new()
        }
    }
}
/*-----------------------------------------------
  Finder searches for text in specified file
*/
#[derive(Debug)]
pub struct Finder {
      dir: String,
      out: GenOut
}
/*-- implement Finder methods -----------------*/
impl Finder {
    pub fn new() -> Finder {
        Finder {
            dir: String::new(),
            out: GenOut::new()
        }
    }
    pub fn set_dir(&mut self, dirnm: &str) {
        self.dir = dirnm.to_string();
        self.out.set_dir(dirnm);
    }
    pub fn find(&mut self, flnm: &str, stxt: &str) {
        /* 
            Pretending to search for text in file.
            Function should:
              1. append flnm to dir
              2. attempt to open file
              3. search for text
              4. send result to out 
        */
        if stxt == "BuildOn" {
            self.out.set_file((flnm, true, stxt));
        }
        else {
            self.out.set_file((flnm, false, stxt));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_genout() {
        let g = GenOut::new();
        assert_eq!(g.dir , "".to_string());
    }
    #[test]
    fn construct_finder() {
        let f = Finder::new();
        assert_eq!(f.dir, "".to_string());
    }
}
