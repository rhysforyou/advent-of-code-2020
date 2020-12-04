use regex::Regex;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParsePasswordError;

impl fmt::Display for ParsePasswordError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "invalid password format")
  }
}

#[derive(Debug)]
pub struct PasswordEntry {
  required_letter: char,
  first_index: usize,
  second_index: usize,
  password: String,
}

impl FromStr for PasswordEntry {
  type Err = ParsePasswordError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
    }

    let captures = RE.captures(s).ok_or(ParsePasswordError)?;

    let first_index = captures
      .get(1)
      .ok_or(ParsePasswordError)?
      .as_str()
      .parse::<usize>()
      .map_err(|_| ParsePasswordError)?
      - 1;

    let second_index = captures
      .get(2)
      .ok_or(ParsePasswordError)?
      .as_str()
      .parse::<usize>()
      .map_err(|_| ParsePasswordError)?
      - 1;

    let required_letter = captures
      .get(3)
      .ok_or(ParsePasswordError)?
      .as_str()
      .chars()
      .nth(0)
      .ok_or(ParsePasswordError)?;

    let password = captures.get(4).ok_or(ParsePasswordError)?.as_str();

    Ok(PasswordEntry {
      required_letter,
      first_index,
      second_index,
      password: String::from(password),
    })
  }
}

impl PasswordEntry {
  pub fn is_valid(&self) -> bool {
    let required_letter_at_first_index =
      self.password.chars().nth(self.first_index) == Some(self.required_letter);
    let required_letter_at_second_index =
      self.password.chars().nth(self.second_index) == Some(self.required_letter);
    (required_letter_at_first_index || required_letter_at_second_index)
      && !(required_letter_at_first_index && required_letter_at_second_index)
  }
}
