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
    let dash_index = s.find('-').ok_or(ParsePasswordError)?;
    let space_index = s.find(' ').ok_or(ParsePasswordError)?;
    let colon_index = s.find(':').ok_or(ParsePasswordError)?;

    let required_letter = s.chars().nth(space_index + 1).ok_or(ParsePasswordError)?;
    let first_index = s[0..dash_index]
      .parse::<usize>()
      .map_err(|_| ParsePasswordError)?
      - 1;
    let second_index = s[dash_index + 1..space_index]
      .parse::<usize>()
      .map_err(|_| ParsePasswordError)?
      - 1;

    Ok(PasswordEntry {
      required_letter: required_letter,
      first_index: first_index,
      second_index: second_index,
      password: String::from(&s[colon_index + 2..s.len()]),
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
