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
  minimum_occurrences: usize,
  maximum_occurrences: usize,
  password: String,
}

impl FromStr for PasswordEntry {
  type Err = ParsePasswordError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let dash_index = s.find('-').ok_or(ParsePasswordError)?;
    let space_index = s.find(' ').ok_or(ParsePasswordError)?;
    let colon_index = s.find(':').ok_or(ParsePasswordError)?;

    let required_letter = s.chars().nth(space_index + 1).ok_or(ParsePasswordError)?;
    let minimum_occurrences = s[0..dash_index]
      .parse::<usize>()
      .map_err(|_| ParsePasswordError)?;
    let maximum_occurrences = s[dash_index + 1..space_index]
      .parse::<usize>()
      .map_err(|_| ParsePasswordError)?;

    Ok(PasswordEntry {
      required_letter: required_letter,
      minimum_occurrences: minimum_occurrences,
      maximum_occurrences: maximum_occurrences,
      password: String::from(&s[colon_index + 2..s.len()]),
    })
  }
}

impl PasswordEntry {
  pub fn is_valid(&self) -> bool {
    let occurrences_count = self.password.matches(self.required_letter).count();
    occurrences_count >= self.minimum_occurrences && occurrences_count <= self.maximum_occurrences
  }
}
