use std::{str::CharIndices, sync::Mutex};

use crate::fatal;

use self::token_tree::TokenTree;
use lazy_static::lazy_static;
pub mod token_tree;
lazy_static! {
    pub static ref FILE: Mutex<String> = Mutex::new(String::from(""));
}
//[pub]
//[pub] static ref NAME_2: TYPE_2 = EXPR_2;//TYPE_1
//...
//[pub] static ref NAME_N: TYPE_N = EXPR_N;
//NAME_1//EXPR_1
#[allow(non_camel_case_types)] //ignore_warning
#[derive(Clone, Debug)]
pub struct Token {
    pub token: Tokens,
    pub line: i32,
    pub start: usize,
    pub end: usize,
    pub snipet: String,
} //lexer
impl Token {
    pub fn from(token: Tokens, line: i32, start: usize, end: usize, snipet: String) -> Token {
        Token {
            token: token,
            line: line,
            start: start,
            end: end,
            snipet: snipet,
        }
    }
}
#[derive(Debug, PartialEq, Clone)] //,Copy
pub enum Tokens {
    //struct

    //Reserved
    PRINT,
    READ,

    //Things
    ASSIGN,

    //Procedures
    PROCE,
    PARAM,

    //Data
    CONST,
    CONST_STR,
    VARIABLE,
    VARIABLE_ASSIGN,
    CLOCK,
   
    //Control
    IF,
    BEGIN,
    END,
    LOOP,
    TOKENS_IF_BEFORE,
    //Logic
    GREATER,
    SMALLER,
    EQUAL,
    NOTEQUAL,
    NEGATION,
    //Math
    ADD,
    SUB,
    DIV,
    MOD,
    MUL,

    //Parenth'eses
    PARGAU,
    PARDRO,
    //EOF & special,
    EOF,
    ERROR(String),
    NONE,
}

pub struct Lexer {
    pub str: String,
    pub start: usize,
    pub index: usize,

    pub tokens: Vec<Token>,
    pub line: i32,
    pub last_token_type: Tokens,
    pub tokentree: TokenTree,
}
impl Lexer {
    pub fn from(d: String) -> Lexer {
        let mut t = FILE.lock().unwrap();
        *t = d.clone();
        //d.clone();
        Lexer {
            //unidecode(
            str: d, //.to_lowercase(), //_
            index: 0,
            tokens: Vec::new(),
            line: 0,
            start: 0,
            last_token_type: Tokens::NONE,
            tokentree: TokenTree::new(),
        }
    }
    pub fn error(&mut self, message: String) -> ! {
        //let n=;
        println!("\nErreur! (Pendant le scannage)");
        println!("{}", message);
        //if let Some(d)=&self.str{//n//text
        let d = self.str.clone();
        let n = d.split("\n"); //pat
                               //let r=self.index;//self.instructions_origins.get(self.instruction).unwrap();
        let mut it = 0;
        let mut ofs = 0;
        let index = self.tokens.last().unwrap().start; //end
        for i in n {
            if it == self.line {
                //r.1
                println!("{}", i); //r.0//index
                for _i in 0..(index - ofs - 1 - 1) {
                    ////self.
                    print!(" ");
                } //r.1//index
                println!(" {} (ligne {})", "^ici", self.line + 1); //{:0>8}//5//'//:0>////i
            }
            ofs += i.len();
            it += 1;
        }
        //}
        //println!("Pour des messages d'erreur plus en profondeur, exécutez \n\tverbose -c -d erreur=oui <fichier>");
        fatal!(); //let _n=
                  //_n
    }
    pub fn lex(&mut self) -> Vec<Token> {
        if DEBUG_LEXER {
            println!("Ligne\tCode\t\t\t\t\tÉquivalent source");
        }
        //println!("JeSuisÉ a: {}",sizeof_char_indices("JeSuisÉ a".char_indices()));//- 1
        //println!("JeSuisE à: {}",sizeof_char_indices("JeSuisE à".char_indices()));//- 1
        //for i in "aei o u1é2é3é4é5é".char_indices(){
        //      println!("{} {}",i.1,i.0);
        //  }
        while !self.at_end() {
            //self.line
            // s!self.at_end()
            //println!("w");

            let oldline = self.line;
            let token = self.scan_token(); //T
            self.tokens.push(token.clone());
            if token.token == self.last_token_type && self.last_token_type == Tokens::VARIABLE {

                //  self.error("Charactère inconnu et/ou innattendu".to_string());
            }
            self.last_token_type = token.token.clone(); //Tok
            if token.line > oldline {
                //||true //||true
                if DEBUG_LEXER {
                    print!("{}\t", token.line + 1);
                }
                self.line = token.line;
            } else {
                if DEBUG_LEXER {
                    print!("|\t");
                }
            }
            if let Tokens::ERROR(d) = &token.token {
                if DEBUG_LEXER {
                    print!("Erreur: {}\n", d);
                }
            } else {
                //let m=&self.str[token.start-1..token.end];
                /*
                 let mut d = self.str.char_indices(); //[token.start-1..token.end];
                 for _ in 0..token.start - 1 {
                     d.next();
                 }
                 let mut c = String::new();
                 if !(self.str.len() <= token.end) {
                     for _ in 0..(token.end - token.start) + 1 {
                         //self.str.len() - token.end-1
                         if let Some(d) = d.next() {
                             c.push(d.1); // /**/=c+&d.1;//d.next();//_back
                         }
                     }
                 } //\t\t\t\t\t\t//40//d.as_str()
                */
                //c
                if DEBUG_LEXER {
                    print!(
                        "{0:<40}\"{1:<0}\"\n",
                        format!("{:?}", token.token),
                        token.snipet
                    ); //m// {}
                }
            }
            if token.token == Tokens::EOF {
                break;
            }
        }
        //let d = self.tokens.clone();
        //self.tokens.clear(); //=Vec::new();//
        self.tokens.clone() //d
    }
    pub fn skip_whitespace_advance(&mut self) -> String {
        //W
        let mut c = self.advance();

        while if let Some(d) = c.char_indices().next() {
            //(
            let d = d.1;
            match d {
                //self.peek(1).chars()
                '/' => {
                    if self.peek(1) == "/" {
                        let m = self.line.clone();
                        while self.line <= m {
                            let m = self.advance();
                            if m == "\n" || m == "" {
                                self.line += 1;
                                c = m;
                            }
                        }
                        true
                    } else {
                        //return false;
                        false
                    }
                }
                '\n' => {
                    ////
                    // println!("NEWLINE");
                    self.line += 1;
                    //| '\n'
                    true
                }
                ' ' | '\t' => true,
                _ => false,
            }
        } else {
            false
        } {
            //)
            //c.trim()
            c = self.advance();
        }
        // c=self.advance();
        return c;
    }
    pub fn scan_token(&mut self) -> Token {
        //T
        if self.at_end() {
            //Token::from//,line
            return self.make_token(Tokens::EOF);
        } //W

        let binding = self.skip_whitespace_advance();
        self.start = self.index.clone();
        let binding2 = binding.clone();
        let mut char = binding2.char_indices();

        if let Some(char) = &char.next() {
            // let mut rd=false;else
            //d//.unwrap()
            let char = char.1;
            match char {
                //| '\n'//.as_str().chars().next().unwrap()
                ' ' => {}
                '/' => {
                    return self.make_token(Tokens::DIV); //t
                }
                '%' => {
                    return self.make_token(Tokens::MOD); //t
                }
                '-' => {
                    return self.make_token(Tokens::SUB); //t
                }
                '+' => {
                    return self.make_token(Tokens::ADD);
                }
                '*' => {
                    return self.make_token(Tokens::MUL);
                }
                '(' => {
                    //println!("D");
                    return self.make_token(Tokens::PARGAU);
                }
                ')' => {
                    //println!("R");
                    return self.make_token(Tokens::PARDRO); //t
                }
                '\"' => {
                    let mut str = String::new(); //self.peek(len)
                                                 //self.advance();
                                                 //self.advance();
                                                 //self.advance();
                    while !self.peek(1).starts_with("\"") {
                        let n = self.peek(2); //len//1
                        if n.starts_with("\\") {
                            //pat

                            if n.ends_with("n") {
                                //println!("D");
                                //println!("ESCAPE CHAR");
                                str += "\n";
                                self.advance();
                                self.advance();
                                continue;
                            }
                        }
                        //pat
                        //println!("{}",str);
                        str += &(self.advance()); //push
                    } //len
                      //self.i
                      //str=str.replacen("\"", "", 1);//if //2
                    self.advance();
                    return self.make_token_str(Tokens::CONST_STR); //t
                }
                //''=>{

                //}
                _t => {
                    if is_digit(_t) {
                        while (match self.peek_1() {
                            '.' => true,
                            _t => {
                                is_digit(_t) //char
                            }
                        }) {
                            self.advance();
                        }
                        return self.make_token(Tokens::CONST);
                        //println!("number");
                    }
                    //char
                    else if _t == '\n' {
                        //&
                        self.line += 1;
                    } else {
                        let mut advlen = 0;
                        let mut token = Tokens::EOF;
                        if let Some(d) = self.tokentree.map_firstchar.get(&_t.to_ascii_lowercase())
                        {
                            //println!("damn!");
                            for i in d {
                                //.size_hint().1.unwrap()
                                let d = format!(
                                    "{}{}",
                                    _t,
                                    self.peek(sizeof_char_indices(i.0.char_indices()) - 1)
                                ); //.to_lowercase(); //0
                                   //println!("{}",d);
                                   //println!("d: [{}] exp: [{}]",d,i.0);
                                   //println!("{} {},",&d,&i.0);
                                if d.eq_ignore_ascii_case(&i.0) {
                                    //other// == i.0
                                    //println!("MATCH!");
                                    token = i.1.clone();
                                    advlen = sizeof_char_indices(i.0.char_indices()) - 1;
                                    //i.0.len() - 1;
                                }
                            }
                            if advlen > 0 {
                                for _i in 0..advlen {
                                    //i.0.len()-1
                                    self.advance();
                                }
                                return self.make_token(token); //Tokens::EOF
                            }
                        }
                        //println!("{}",d);
                    } //se //else
                    if _t.is_alphabetic() {
                        //_whitespace//numeric//!//'
                        while (self.peek_1().is_alphanumeric()) {
                            self.advance();
                        }
                        return self.make_token(Tokens::VARIABLE);
                    }
                }
            }
        } else {
            //println!("O NO");////
            return self.make_token(Tokens::EOF); //;
        }

        return self//|       EQUAL                                   "est égal à 1"
            .error_token(&(String::from("Charactère inconnu '") + &binding + &String::from("'")));
    }
    pub fn get_chunk(&self, start: usize, end: usize) -> String {
        let mut k = String::new();
        let mut it = self.str.char_indices();
        for _i in 0..start - 1 {
            it.next();
        }
        for _i in 0..end - start + 1 {
            if let Some(d) = it.next() {
                k.push(d.1);
            }
        }
        k
    }
    pub fn error_token(&mut self, text: &str) -> Token {
        Token {
            token: Tokens::ERROR(text.to_string()),
            line: self.line, // radix
            start: self.start,
            end: self.index, //d
            snipet: self.get_chunk(self.start, self.index),
        }
    }
    pub fn make_token_str(&mut self, t: Tokens) -> Token {
        Token {
            token: t,
            line: self.line,
            start: self.start + 1,
            end: self.index - 1,
            snipet: self.get_chunk(self.start + 1, self.index - 1),
        }
    }
    pub fn make_token(&mut self, t: Tokens) -> Token {
        Token {
            token: t,
            line: self.line,
            start: self.start,
            end: self.index,
            snipet: self.get_chunk(self.start, self.index),
        }
    }

    pub fn advance(&mut self) -> String {
        self.index += 1; //+
        if self.at_end() {
            return String::from("");
        } //[self.index - 1..self.index]
        if let Some(d) = self.str.char_indices().nth(self.index - 1) {
            //+1
            //  println!("{} {}",d.1,self.str[self.index - 1..self.index].to_string());
            return d.1.to_string(); //;bg!()
        } // self.str[self.index - 1..self.index].to_string();
        return String::from("");
        //return.unwrap()
    }
    pub fn at_end(&self) -> bool {
        return self.index > self.str.len(); //=
    }
    pub fn peek_1(&self) -> char {
        //, len: usize
        //=//String
        /*
        if self.at_end() || self.index + len > self.str.len() {
            return String::from("");
        } else {
            //1
            let mut k=self.str.char_indices();
            for _i in 0..self.index{//k
                k.next();
            }
            let mut n=String::new();//len
            for _i in 0..len{
                if let Some(d)=k.next(){
                    n.push(d.1);
                }
            }
            //println!("{} {}",self.str[self.index..self.index + len].to_string(),n);
            return n;
           // if let Some(d)= self.str.char_indices(){//.nth(self.index)
           //     return d.1.to_string();//;bg!()
          //  }// self.str[self.index - 1..self.index].to_string();
          //  return self.str[self.index..self.index + len].to_string();
        }
         */
        let k = self.peek(1);
        if let Some(d) = k.char_indices().next() {
            return d.1;
        }
        ' '
    }
    pub fn peek(&self, len: usize) -> String {
        //=
        if self.at_end() || self.index + len > self.str.len() {
            return String::from("");
        } else {
            //1
            let mut k = self.str.char_indices();
            for _i in 0..self.index {
                //k
                k.next();
            }
            let mut n = String::new(); //len
            for _i in 0..len {
                if let Some(d) = k.next() {
                    n.push(d.1);
                }
            }
            //println!("{} {}",self.str[self.index..self.index + len].to_string(),n);
            return n;
            // if let Some(d)= self.str.char_indices(){//.nth(self.index)
            //     return d.1.to_string();//;bg!()
            //  }// self.str[self.index - 1..self.index].to_string();
            //  return self.str[self.index..self.index + len].to_string();
        }
    }
}
pub fn lex(str: String) -> Lexer {
    //&str
    let mut m = Lexer::from(str); //d
    let _d = m.lex(); //path
                      //println!("{}",_d.len());
    m
}
pub fn is_digit(char: char) -> bool {
    //println!("{}")
    return char.is_digit(10); //radix
                              //return char>='0'&&char<='9';//""
}

/*
Exemple de sortie: (morceau de) 17:10 25 Août 2023
|       ASSIGN                                  "vaut"
|       Erreur: Charactère inconnu '5'
|       Erreur: Charactère inconnu '0'
3       PRINT                                   "afficher"
 */

pub fn sizeof_char_indices(t: CharIndices) -> usize {
    let mut size = 0;
    let mut t = t.clone();
    while t.next().is_some() {
        size += 1;
    }
    size
}

pub const DEBUG_LEXER: bool = false;
