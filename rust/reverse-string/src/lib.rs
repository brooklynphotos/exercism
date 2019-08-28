extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  let mut g = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
  g.reverse();
  let mut r_string = String::new();
  for s in g.iter() {
    r_string.push_str(s);
  }
  r_string
}
