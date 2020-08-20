extern crate regex;

use std::io;
use super::tokens;
use regex::Regex;

pub struct Lexer {
  reserved_word: tokens::TokenToIssue,
  pub tokens: tokens::TokenToIssue,
  value: String,
  index: usize,
}

impl Lexer {
  pub fn new(value: &str) -> Self {
    Lexer {
      reserved_word: tokens::TokenToIssue::new(),
      tokens: tokens::TokenToIssue::new(),
      value: value.to_string(),
      index: 0,
    }
  }

  pub fn run(&mut self)-> &tokens::TokenToIssue {
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
      Some(_) => loop {
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
        for token in self.reserved_word.get_tokens().iter() {
          if identifier_str == token.get_value() {
            return token.clone();
          }
        }
      },
      None => {}
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

  pub fn push_token(&mut self, token: i64, value: &str) -> Result<(), String> {
    if token > 0 {
      return Err("The argument must be a negative number".to_string());
    }
    self.reserved_word.push(token, value);
    return Ok(());
  }

  fn push_tokens(&mut self, tokens: &tokens::Tokens) {
    self.tokens.tokens_push(tokens);
  }
}