extern crate regex;
use super::tokens;
use regex::Regex;

pub struct Lexer {
  pub tokens: tokens::TokenToIssue,
  reserved_word: tokens::TokenToIssue,
  between_word: tokens::TokenToIssue,
  one_word: tokens::TokenToIssue,
  other_ward_token: i64,
  number_token: i64,
  value: String,
  index: usize,
}

impl Lexer {
  pub fn new(value: &str) -> Self {
    Lexer {
      reserved_word: tokens::TokenToIssue::new(),
      between_word: tokens::TokenToIssue::new(),
      tokens: tokens::TokenToIssue::new(),
      one_word: tokens::TokenToIssue::new(),
      other_ward_token: 0,
      value: value.to_string(),
      number_token: 0,
      index: 0,
    }
  }

  pub fn run(&mut self) -> &tokens::TokenToIssue {
    let len = self.value.len();
    loop {
      if self.index >= len {
        break;
      }

      let result = self.get();
      self.push_tokens(&result);
    }

    return &self.tokens;
  }

  fn get(&mut self) -> tokens::Tokens {
    let mut one_char = self.value.chars().nth(self.index);
    while one_char == Some('\n') || one_char == Some(' ') {
      self.index += 1;
      one_char = self.value.chars().nth(self.index);
    }

    let len = self.value.len();
    if self.index >= len {
      return tokens::Tokens::new(0, "");
    }

    let last_str = self
      .value
      .chars()
      .nth(self.index)
      .expect("Failed")
      .to_string();

    let mut identifier_str: String = String::new();

    let reg = Regex::new(r"[a-zA-Z]+").expect("Failed");
    match reg.captures(&last_str) {
      Some(_) => {
        loop {
          let text = &self
            .value
            .chars()
            .nth(self.index)
            .expect("Failed")
            .to_string();
          let reg = Regex::new(r"(\d|[a-zA-Z])+").expect("Failed");

          let res = match reg.captures(text) {
            Some(_) => true,
            None => false,
          };

          if !res {
            break;
          }

          identifier_str += text;
          self.index += 1;

          if self.index >= len {
            break;
          }
        }

        if !self.reserved_word.get_tokens().is_empty() {
          for token in self.reserved_word.get_tokens().iter() {
            if identifier_str == token.get_value() {
              return token.clone();
            }
          }
        }

        if self.other_ward_token < 0 {
          return tokens::Tokens::new(self.other_ward_token, &identifier_str);
        }
      }
      None => {}
    }

    if !self.between_word.tokens.is_empty() {
      for token in self.between_word.get_tokens().iter() {
        let reg = Regex::new(&format!(r#"{}"#, token.value)).expect("Failed");
        match reg.captures(&last_str) {
          Some(_) => {
            identifier_str = String::new();
            loop {
              self.index += 1;
              let text = &self
                .value
                .chars()
                .nth(self.index)
                .expect("Failed")
                .to_string();
              if text != &token.value {
                identifier_str += &text
              }

              if text == &token.value {
                break;
              }
            }

            self.index += 1;
            let token_value = tokens::Tokens::new(token.token, &identifier_str);
            return token_value;
          }

          None => {}
        }
      }
    }

    if !self.one_word.tokens.is_empty() {
      for token in self.one_word.get_tokens().iter() {
        let reg = Regex::new(&format!(r#"{}"#, token.value)).expect("Faild");
        match reg.captures(&last_str) {
          Some(_) => {
            loop {
              let text = &self
                .value
                .chars()
                .nth(self.index)
                .expect("Failed")
                .to_string();
              let len = self.value.len();
              if text == "\n" {
                break;
              }

              identifier_str += text;
              self.index += 1;

              if self.index >= len {
                break;
              }
            }
            let token_value = tokens::Tokens::new(token.token, &identifier_str);
            return token_value;
          }
          None => {}
        }
      }
    }

    if self.number_token < 0 {
      let reg = Regex::new(r"[0-9]+").expect("Faild");
      match reg.captures(&last_str) {
        Some(_) => {
          loop {
            let text = &self
              .value
              .chars()
              .nth(self.index)
              .expect("Failed")
              .to_string();
            let reg = Regex::new(r"[0-9.]+").expect("Faild");
            let res = match reg.captures(text) {
              Some(_) => true,
              None => false,
            };
            let len = self.value.len();
            if !res {
              break;
            }

            identifier_str += text;
            self.index += 1;

            if self.index >= len {
              break;
            }
          }

          let token_value = tokens::Tokens::new(self.number_token, &identifier_str);
          return token_value;
        }
        None => {}
      };
    }

    let ascii_code = self
      .value
      .chars()
      .nth(self.index)
      .expect("Failed")
      .to_string()
      .as_bytes()[0];
    let token_value = tokens::Tokens::new(ascii_code as i64, &last_str);
    self.index += 1;
    return token_value;
  }

  pub fn push_reserved_word(&mut self, token: i64, value: &str) -> Result<(), String> {
    if token > 0 {
      return Err("The argument must be a negative number".to_string());
    }
    self.reserved_word.push(token, value);
    return Ok(());
  }

  pub fn push_between_ward(&mut self, token: i64, value: &str) -> Result<(), String> {
    if token > 0 {
      return Err("The argument must be a negative number".to_string());
    }
    self.between_word.push(token, value);
    return Ok(());
  }

  pub fn set_number_token(&mut self, token: i64) -> Result<(), String> {
    if token > 0 {
      return Err("The argument must be a negative number".to_string());
    }

    self.number_token = token;
    return Ok(());
  }

  pub fn set_other_token(&mut self, token: i64) -> Result<(), String> {
    if token > 0 {
      return Err("The argument must be a negative number".to_string());
    }

    self.other_ward_token = token;
    return Ok(());
  }

  pub fn push_one_word(&mut self, token: i64, value: &str) -> Result<(), String> {
    if token > 0 {
      return Err("The argument must be a negative number".to_string());
    }

    self.one_word.push(token, value);
    return Ok(());
  }

  fn push_tokens(&mut self, tokens: &tokens::Tokens) {
    self.tokens.tokens_push(tokens);
  }
}
