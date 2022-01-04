
use crate::scanner::ScannerRead;
use crate::scanner::ScannerCreate;
use crate::scanner::Scanner;
use crate::token::Token;

use std::path::Path;
use std::fs::File;

pub struct Lexer {
    scanner:Scanner,
    cache_tokens:std::vec::Vec<Token>
}

pub trait LexerCreate {
    fn from_file(file: File) -> Lexer;
    fn from_string(s:&'static str) -> Lexer;
    fn from_file_name(path: &'static Path) -> Lexer;
}

pub trait LexerTokenizer {
    fn get_token(&mut self)->Token;
    fn push_token(&mut self,t:Token);
}

trait LexerScan {
    fn forward_file(&mut self, position: usize);
    fn read_pos(&mut self, position:usize )->u8 ;
    fn forward_pos(&mut self, position:usize ) ;
}

impl LexerCreate for Lexer {
    fn from_file(file: File) -> Lexer {
        return  Lexer { scanner: Scanner::from_file(file), cache_tokens:Vec::new() };
    }
    fn from_string(s:&'static str) -> Lexer {
      
        return  Lexer { scanner: Scanner::from_string(s), cache_tokens:Vec::new() };
    }    
    fn from_file_name(path: &'static Path) -> Lexer {
        return  Lexer { scanner: Scanner::from_file_name(path), cache_tokens:Vec::new() };
    }
}

impl LexerTokenizer for Lexer {
    fn push_token(&mut self , t: Token){
        self.cache_tokens.push(t)
    }    
    fn get_token(&mut self)-> Token {
        if self.cache_tokens.is_empty() {
            self.cache_tokens= self.scanner.read()
        }
        self.cache_tokens.remove(0)
    }
}


#[cfg(test)]
mod tests {
    use crate::token::TokenCreate;
    use crate::token::Token;
    use crate::lexer::LexerTokenizer;
    use crate::lexer::LexerCreate;
    use crate::lexer::Lexer;    
    #[test]
    fn sequence_integer() {
        let s = "1234567 1234567 12345";
        let mut reader =Lexer::from_string(s);
        assert_eq!(reader.get_token(),Token::create_token_numeric("1234567",0,7));
        assert_eq!(reader.get_token(),Token::create_token_numeric("1234567",8, 7));
        assert_eq!(reader.get_token(),Token::create_token_numeric("12345",16, 5));
        assert_eq!(reader.get_token(),Token::create_token_eof(s.len()));
    }
    #[test]
    fn sequence_string() {

        let s = "'12345' \"12345\" '123'";
        let mut reader =Lexer::from_string(s);
        assert_eq!(reader.get_token(),Token::create_token_char("12345",0,7));
        assert_eq!(reader.get_token(),Token::create_token_string("12345",8, 7));
        assert_eq!(reader.get_token(),Token::create_token_char("123",16, 5));
        assert_eq!(reader.get_token(),Token::create_token_eof(s.len()));
    }
    
    #[test]
    fn sequence_integer_string() {
        let s = "'12345' 2342365 \"123\"";
        let mut reader =Lexer::from_string(s);
        assert_eq!(reader.get_token(),Token::create_token_char("12345",0,7));
        assert_eq!(reader.get_token(),Token::create_token_numeric("2342365",8, 7));
        assert_eq!(reader.get_token(),Token::create_token_string("123",16, 5));
        assert_eq!(reader.get_token(),Token::create_token_eof(s.len()));
    }

    #[test]
    fn sequence_ident() {
        let s = "work1 work2 work3";
        let mut reader =Lexer::from_string(s);
        assert_eq!(reader.get_token(),Token::create_token_ident("work1",0,5));
        assert_eq!(reader.get_token(),Token::create_token_ident("work2",6, 5));
        assert_eq!(reader.get_token(),Token::create_token_ident("work3",12, 5));
        assert_eq!(reader.get_token(),Token::create_token_eof(s.len()));
    }
}