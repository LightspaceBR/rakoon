#[derive(Debug)]
pub enum TokenConst {
    Char(String),
    String(String),
    Numeric(String)
}

#[derive(Debug)]
pub enum TokenValue {
    Eof,
    InvalidChar(String),
    Const(TokenConst),
    Ident(String)
}

#[derive(Debug)]
pub  struct Token {
    value:TokenValue,
    pos:usize,
    len:usize
}

pub trait TokenCreate {
    fn create(value:TokenValue,pos:usize,len:usize) -> Token;
    fn create_token_const(value:TokenConst,pos:usize,len:usize) -> Token;
    fn create_token_string(value:&str,pos:usize,len:usize) -> Token;
    fn create_token_char(value:&str,pos:usize,len:usize) -> Token;
    fn create_token_ident(value:&str,pos:usize,len:usize) -> Token;
    fn create_token_eof(pos:usize) -> Token;
    fn create_invalid_char(value:String,pos:usize) -> Token;
    fn create_token_numeric(value:&str,pos:usize,len:usize) -> Token;
}

impl PartialEq for TokenValue {
    fn eq(&self, other: &TokenValue) -> bool {
        match (self,other ){
            (TokenValue::Eof,TokenValue::Eof) => true,
            (TokenValue::Const(v1),TokenValue::Const(v2)) => v1==v2,
            (TokenValue::Ident(v1),TokenValue::Ident(v2)) => v1==v2,
            (TokenValue::InvalidChar(v1),TokenValue::InvalidChar(v2)) => v1==v2,
            _ =>false
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.pos==other.pos &&  self.value==other.value&&  self.len==other.len
    }
}

impl PartialEq for TokenConst {
    fn eq(&self, other: &TokenConst) -> bool {
        match (self,other ){
            (TokenConst::Numeric(v1),TokenConst::Numeric(v2)) => v1==v2,
            (TokenConst::String(v1),TokenConst::String(v2)) => v1==v2,
            (TokenConst::Char(v1),TokenConst::Char(v2)) => v1==v2,
            _ =>false
        }
    }
}


impl TokenCreate for Token {
    fn create(value:TokenValue,pos:usize,len:usize) -> Token {
        Token {value:value,pos:pos,len:len}
    }
    fn create_token_const(value:TokenConst,pos:usize,len:usize) -> Token {
        Token::create(TokenValue::Const(value),pos,len)
    }    
    fn create_token_string(value:&str,pos:usize,len:usize) -> Token {
        Token::create_token_const(TokenConst::String(value.to_string()),pos,len)
    }
    fn create_token_char(value:&str,pos:usize,len:usize) -> Token {
        Token::create_token_const(TokenConst::Char(value.to_string()),pos,len)
    }    
    fn create_token_numeric(value:&str,pos:usize,len:usize) -> Token {
        Token::create_token_const(TokenConst::Numeric(value.to_string()),pos,len)
    }    
    fn create_token_eof(pos:usize) -> Token {
        Token::create(TokenValue::Eof,pos,0)
    }    
    fn create_invalid_char(value:String,pos:usize) -> Token {
        Token::create(TokenValue::InvalidChar(value),pos,1)
    }
    
    fn create_token_ident(value:&str,pos:usize,len:usize) -> Token{
        Token::create(TokenValue::Ident(value.to_string()),pos,len)
    }
}

