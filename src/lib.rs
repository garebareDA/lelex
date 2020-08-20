pub mod lexers;
pub mod tokens;

#[cfg(test)]
mod tests {
    use super::lexers;
    #[test]
    fn lex_if(){
        let mut lex = lexers::Lexer::new("if");
        lex.push_reserved_word(-1, "if").expect("Fail");
        let result = lex.run();
        let tokens = result.get_tokens();
        assert_eq!(tokens[0].token, -1);
    }

    #[test]
    fn error_test(){
        let mut lex = lexers::Lexer::new("if");
        match lex.push_reserved_word(1, "if") {
            Ok(()) => {
                panic!("error handle faild");
            }
            Err(err)=>{
                println!("{}", err);
            }
        }
    }

    #[test]
    fn between(){
        let mut lex = lexers::Lexer::new("\"if\"");
        lex.push_between_ward(-1, "\"").expect("failed");
        let result = lex.run();
        let tokens = result.get_tokens();
        assert_eq!(tokens[0].token, -1);
        assert_eq!(tokens[0].value, "if");
    }

    #[test]
    fn one(){
        let mut lex = lexers::Lexer::new("#test");
        lex.push_one_word(-1, "#").expect("Failed");
        let result = lex.run();
        let tokens = result.get_tokens();
        assert_eq!(tokens[0].token, -1);
    }

    #[test]
    fn number(){
        let mut lex = lexers::Lexer::new("111");
        lex.set_number_token(-1).expect("Failed");
        let result = lex.run();
        let tokens = result.get_tokens();
        assert_eq!(tokens[0].token, -1);
    }
}