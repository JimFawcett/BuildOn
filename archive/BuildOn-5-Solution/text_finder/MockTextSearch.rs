// #[derive(Debug)]
// pub struct MockTextSearch {
//     curr_path: PathBuf,
//     path_displayed: bool,
//     display_unmatched: bool
// }
// impl DirEvent for MockTextSearch {
//     fn new() -> Self {
//         Self {
//             curr_path: PathBuf::new(),
//             path_displayed: false,
//             display_unmatched: false
//         }
//     }
//     fn do_dir(&mut self, d: &Path) {
//         self.curr_path = d.to_path_buf();
//         if self.display_unmatched {
//             print!("\n  {:?}", replace_sep(&self.curr_path));
//         }
//         self.path_displayed = false;
//     }
//     fn do_file(&mut self, f: &Path) {
//         if !self.path_displayed && !self.display_unmatched {
//             print!("\n    {:?}", replace_sep(&self.curr_path));
//         }
//         print!("\n    {:?}", f);
//     }
// }
// impl Default for MockTextSearch {
//     fn default() -> MockTextSearch {
//         MockTextSearch::new()
//     }
// }
// impl MockTextSearch {
//     pub fn set_hide(&mut self, do_hide:bool) {
//         self.display_unmatched = !do_hide;
//     }
// }
