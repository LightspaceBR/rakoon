

use crate::helps::ToString;
use std::collections::HashSet;
use crate::token::TokenCreate;
use crate::token::Token;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::io::Read;


pub struct Scanner {
    file:Box<dyn Read>,
    cache_chars:std::vec::Vec<u8>,
    pub pos:usize
}

pub trait ScannerCreate {
    fn from_file(file: File) -> Scanner;
    fn from_string(s:&'static str) -> Scanner;
    fn from_file_name(path: &'static Path) -> Scanner;
}

pub trait ScannerTokenizer {
    fn get_token(&mut self)->Token;
    fn push_token(&mut self,t:Token);
}

pub trait ScannerScan {
    fn forward_file(&mut self, position: usize);
    fn read_pos(&mut self, position:usize )->u8 ;
    fn forward_pos(&mut self, position:usize ) ;
}

pub trait ScannerRead {
    fn read(&mut self)->std::vec::Vec<Token>;
    fn jump_space(&mut self)->std::vec::Vec<Token>;
    fn read_char(&mut self)->std::vec::Vec<Token>;
    fn read_ident(&mut self)->std::vec::Vec<Token>;
    fn read_invalid_char(&mut self)->std::vec::Vec<Token>;
    fn read_numeric(&mut self)->std::vec::Vec<Token>;
    fn read_eof(&mut self)->std::vec::Vec<Token>;
    fn read_string(&mut self)->std::vec::Vec<Token>;  
    fn make_string(&mut self)-> (String,usize,usize);   
}

fn div_ceil(i:usize,j:usize)->usize {
    if i % j == 0 {
        return i/j;
    }
    return i/j + 1;
}
 impl ScannerCreate for Scanner {
    fn from_file(file: File) -> Scanner {
        return  Scanner { file:Box::new(file),cache_chars:Vec::new(),pos:0};
    }
    fn from_string(s:&'static str) -> Scanner {
        return  Scanner { file:Box::new(BufReader::new(s.as_bytes())),cache_chars:Vec::new(),pos:0};
    }    
    fn from_file_name(path: &'static Path) -> Scanner {
        return Scanner::from_file(File::open(path).expect(""));
    }
}

impl ScannerScan for Scanner {
    fn read_pos(&mut self, position: usize  ) ->u8 {
        self.forward_file(position);
        return self.cache_chars[position];
    }
    fn forward_pos(&mut self, position: usize  ) {
        self.forward_file(position);
        for _ in 0..position {
           if self.cache_chars[0] != 0{
             self.pos = self.pos + 1 
           }
           
           self.cache_chars.remove(0);
        }
        
    }
    fn forward_file(&mut self, position: usize){
        let l = position+1;
        let c_len =   self.cache_chars.len();
        if l > c_len{
            let ii = div_ceil(l-c_len,8);
            for _ in 0..ii {
                let mut buffer = [0 ; 8];
                self.file.read(&mut buffer[..]).expect("error read file");
                self.cache_chars.extend_from_slice(&buffer);
            }
        }
    }
}


lazy_static! {
    static ref IDENT: HashSet<u8> = {
        let ident_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_".chars();
        let mut map = HashSet::new();
        for elem in ident_chars {
            map.insert(
                 elem as u8,
             );
         }
         map
    };
}


impl ScannerRead for Scanner {
    fn read(&mut self) ->std::vec::Vec<Token> {
        let c=self.read_pos(0);
        return match (c, c as char) {
            (0,_)=>  self.read_eof(),
            (32|9|13,_)=>self.jump_space(),
            (_,'"')=>self.read_string(),
            (_,'\'')=>self.read_char(),
            (48..=57,_)=>self.read_numeric(),
            (c,_ ) if IDENT.contains(&c) =>self.read_ident(),
            _=>self.read_invalid_char(),
        }
    }
    fn jump_space(&mut self)->std::vec::Vec<Token>{
        self.forward_pos(1);
        self.read()
    }
    fn read_ident(&mut self)->std::vec::Vec<Token>{
        let mut ii=0;
        let pos = self.pos;
        let mut c=self.read_pos(0);
        let mut buf:std::vec::Vec<char> = Vec::new();
        buf.push(c as char);
        loop {
            ii=ii+1;
            c = self.read_pos(ii);
            if IDENT.contains(&c) {
                buf.push(c as char);
            }else {
                break;
            }
        }
        self.forward_pos(ii+1);
        vec![Token::create_token_ident(&buf.to_string(),pos,ii)]
    }
    fn read_invalid_char(&mut self)->std::vec::Vec<Token>{
        let c=self.read_pos(0);
        self.forward_pos(1);
        vec![Token::create_invalid_char(c.to_string(),self.pos)]
    }
    fn read_eof(&mut self)->std::vec::Vec<Token>{
        vec![Token::create_token_eof(self.pos)]
    }    
    fn read_char(&mut self)->std::vec::Vec<Token>{
        let (stuff_str,pos,ii) = self.make_string();
        vec![(Token::create_token_char(&stuff_str,pos,ii))]
    }     
    fn read_string(&mut self)->std::vec::Vec<Token>{
        let (stuff_str,pos,ii) =self.make_string();
        vec![Token::create_token_string(&stuff_str,pos,ii)]
    }       
    fn read_numeric(&mut self)->std::vec::Vec<Token>{
        let mut ii=0;
        let pos = self.pos;
        let mut c=self.read_pos(0);
        let mut buf:std::vec::Vec<char> = Vec::new();
        buf.push(c as char);
        loop {
            ii=ii+1;
            c = self.read_pos(ii);
            match c  {
                48..=57 =>{
                    buf.push(c as char);
                },
                _ => break

            }
        }
        self.forward_pos(ii+1);
        vec![Token::create_token_numeric(&buf.to_string(),pos,ii)]
    }
    
    fn make_string(&mut self)-> (String,usize,usize){
        let mut ii=0;
        let pos = self.pos;
        let char_end=self.read_pos(0);
        let mut buf:std::vec::Vec<char> = Vec::new();
        loop {
            ii=ii+1;
            let c= self.read_pos(ii);
            match (c, c as char)  {
                (0,_) => {
                    break;
                }
                (_,'\\' ) => {
                    ii=ii+1;
                    let c2= self.read_pos(ii);
                    match c2 as char {
                        'n' => {
                            buf.push('\n');
                        }
                        't' => {
                            buf.push('\t');
                        }
                        'r' => {
                            buf.push('\r');
                        }
                        _ => {
                            buf.push(c as char);
                        }
                    }
                    ii=ii+1;
                }
                (c ,_) if c == char_end =>{
                   ii=ii+1;
                   break;
                },
                _ => {
                    buf.push(c as char);
                }
            }
        }
        self.forward_pos(ii+1);
        return (buf.to_string(),pos,ii)
    } 
}