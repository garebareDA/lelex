pub mod lexers;
pub(crate) mod tokens;

#[cfg(test)]
mod tests {
    use super::lexers;
    #[test]
    fn lex_if(){
        let mut lex = lexers::Lexer::new("if");
        lex.push_token(-1, "if").expect("Fail");
        let result = lex.run();
        let tokens = result.get_tokens();
        assert_eq!(tokens[0].token, -1);
    }

    #[test]
    fn error_test(){
        let mut lex = lexers::Lexer::new("if");
        match lex.push_token(1, "if") {
            Ok(()) => {
                panic!("error handle faild");
            }
            Err(err)=>{
                println!("{}", err);
            }
        }
    }
}