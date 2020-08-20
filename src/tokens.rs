pub struct TokenToIssue{
  pub(crate) tokens:Vec<Tokens>
}

#[derive(Clone)]
pub struct Tokens{
  pub token: i64,
  pub value:String
}

impl TokenToIssue{
  pub fn new() -> Self{
    TokenToIssue{
      tokens: Vec::new(),
    }
  }

  pub fn get_tokens(&self) -> &Vec<Tokens>{
    &self.tokens
  }

  pub fn push(&mut self, token:i64, value:&str) {
    let token = Tokens::new(token, value);
    self.tokens.push(token);
  }

  pub fn tokens_push(&mut self, result :&Tokens){
    self.tokens.push(result.clone());
  }
}

impl Tokens{
  pub fn new(token:i64, value:&str) -> Self{
    Tokens{
      token: token,
      value: value.to_string()
    }
  }

  pub fn get_tokens(&self) -> i64{
    self.token
  }

  pub fn get_value(&self) -> &str{
    &self.value
  }
}