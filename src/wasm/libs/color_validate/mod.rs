use regex::RegexSet;

pub fn is_valid_hex(hex: &str) -> bool {
  lazy_static! {
    static ref HEX: RegexSet =
      RegexSet::new(&[r"^#?([\da-fA-F]{3})$", r"^#?([\da-fA-F]{6})$"]).unwrap();
  }

  HEX.is_match(hex)
}
