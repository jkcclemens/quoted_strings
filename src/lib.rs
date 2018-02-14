#[cfg(test)]
mod test;

use std::str::Chars;

pub struct QuotedParts<'a> {
  inner: Chars<'a>,
  extra: Option<Vec<String>>
}

impl<'a> QuotedParts<'a> {
  pub fn from(input: &'a str) -> Self {
    QuotedParts {
      inner: input.chars(),
      extra: None
    }
  }

  pub fn all(input: &'a str) -> Vec<String> {
    QuotedParts::from(input).collect()
  }

  fn parse_escapes(input: &str) -> String {
    let mut result = String::new();

    let mut escaped = false;
    for c in input.chars() {
      if escaped {
        result.push(c);
        escaped = false;
        continue;
      }

      if c == '\\' {
        escaped = true;
        continue;
      }

      result.push(c);
    }

    result
  }

  fn extra(&mut self) -> Option<String> {
    let extra = self.extra.as_mut()?;
    if extra.len() == 0 {
      return None;
    }
    Some(extra.remove(0))
  }
}

impl<'a> Iterator for QuotedParts<'a> {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(extra) = self.extra.as_mut() {
      if extra.is_empty() {
        return None;
      }
      return Some(QuotedParts::parse_escapes(&extra.remove(0)));
    }

    let mut current = String::new();
    let mut inside = String::new();
    let mut in_quote = false;

    for c in &mut self.inner {
      if in_quote {
        if c == '"' {
          return Some(QuotedParts::parse_escapes(&inside[1..]));
        }
        inside.push(c);
        continue;
      }

      if c == '"' {
        in_quote = true;
        if !current.is_empty() {
          inside += &current;
          current.clear();
        }
        inside.push(c);
        continue;
      }

      if c == ' ' {
        if !current.is_empty() {
          return Some(QuotedParts::parse_escapes(&current));
        }
        continue;
      }

      current.push(c);
    }

    if !inside.is_empty() {
      self.extra = Some(inside.split(' ').filter(|x| !x.is_empty()).map(ToOwned::to_owned).collect());
    }

    if !current.is_empty() {
      return Some(QuotedParts::parse_escapes(&current));
    }

    self.extra()
  }
}
