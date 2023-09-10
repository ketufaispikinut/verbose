use crate::{
    fatal,
    lexer::{self, Lexer, Token, Tokens},
};

pub mod token_precedence;
use token_precedence::infix_binding_power;

use self::token_precedence::prefix_binding_power;
pub struct TokenBox {
    pub vec: Vec<Token>,
    pub index: usize,
}

impl TokenBox {
    pub fn next(&mut self) -> Token {
        //s
        self.index += 1;
        return self.vec[self.index - 1].clone(); //.token
    }
    pub fn peek(&mut self) -> Token {
        //-//s//+ 1
        return self.vec[self.index].clone(); //.token
    }
    pub fn is_at_end(&self) -> bool {
        return self.index >= self.vec.len();
    }
    pub fn consume(&mut self, token: Tokens, message: &str) {
        if self.peek().token == token {
            self.next();
        } else {
            fatal!("{}", message);
        }
    }
}
pub fn parse_rearrange(lexer: &Lexer) -> Vec<Token> {
    //_to_instructions
    let mut k = TokenBox {
        vec: lexer.tokens.clone(),
        index: 0,
    };
    let mut vec_of_tokens = Vec::new(); //c
    while !&k.is_at_end() {
        while isop(&k.peek().token) {
            let mut c = back_to_tokens(parse(&mut k, 0),false); //
            vec_of_tokens.append(&mut c.0);
        }
        if !k.is_at_end() {
            let d = k.next();
            //if d.token==Tokens::BEGIN{
            //
            // }
            // else{
            if d.token != Tokens::BEGIN {
                vec_of_tokens.push(d); //append//o//&mut
            }
            //}
        }
    }
    if DEBUG_PARSER {
        println!("{}", dis(&vec_of_tokens)); //:?
    }
    vec_of_tokens
}
pub fn isop(t: &Tokens) -> bool {
    match *t {
        Tokens::ADD
        | Tokens::CONST
        | Tokens::CONST_STR
        | Tokens::DIV
        | Tokens::MOD
        | Tokens::PARDRO
        | Tokens::PARGAU
        | Tokens::MUL
        | Tokens::SUB
        | Tokens::ASSIGN
        | Tokens::VARIABLE
        | Tokens::GREATER
        | Tokens::SMALLER
        | Tokens::EQUAL 
        | Tokens::PRINT
        | Tokens::IF//,
        | Tokens::LOOP//,
        => true,
        _ => false,
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    NUM, // I32,//i//MATH
    // F32,//f
    BOOL,
    STR,
    ANY,
}
#[derive(Debug)]
pub enum ParseResult {
    Atom(Token, Type),             //vec!//[//]
    Cons(Token, Vec<ParseResult>), //,Type
}
fn parse(v: &mut TokenBox, min_power: u8) -> ParseResult {
    //pub //Vec<Token>
    //println!("PARSE CALL");
    //println!("DONG");
    let f = v.next();
    let mut lhs = match f.token.clone() {
        Tokens::CONST => ParseResult::Atom(
            f.clone(),
            match is_bool(f.snipet.clone()) {
                true => Type::BOOL,
                _ => Type::NUM,
            },
        ),
        Tokens::CONST_STR => ParseResult::Atom(f, Type::STR),
        Tokens::VARIABLE => ParseResult::Atom(f, Type::ANY),
        Tokens::CLOCK=>ParseResult::Atom(f,Type::NUM),
        Tokens::LOOP => {
            let ((), r_bp) = prefix_binding_power('w').unwrap(); //p
            let rhs = parse(v, r_bp); //expr_bp
            return ParseResult::Cons(f, vec![rhs]); //lexer//op//S
        }
        Tokens::PRINT => {
            //op
            //println!("print");
            let ((), r_bp) = prefix_binding_power('p').unwrap();
            let rhs = parse(v, r_bp); //expr_bp
            return ParseResult::Cons(f, vec![rhs]); //lexer//op//S
        }
        Tokens::IF => {
            let ((), r_bp) = prefix_binding_power('i').unwrap(); //p
            let rhs = parse(v, r_bp); //expr_bp
            return ParseResult::Cons(f, vec![rhs]); //lexer//op//S
        }
        Tokens::PARGAU => {
            //Token::Op('(') => {
            //println!("DAMN");
            let lhs = parse(v, 0); //lexer
                                   //assert_eq!(lexer.next(), Token::Op(')'));
            v.consume(
                Tokens::PARDRO,
                &"Une parenthèse fermante est ici nécessaire",
            ); //lexer
            lhs //return lhs;
                // }
        }
        t_ => {
            fatal!("o non (lhs): {:?}", t_);
        }
    }; //
    loop {
        // println!("NEXT");//peek//next
        let d = v.peek();
        let op = match d.token {
            Tokens::EOF => break,
            Tokens::ADD => '+',
            Tokens::SUB => '-',
            Tokens::MUL => '*',
            Tokens::DIV => '/',
            Tokens::MOD => '%',
            Tokens::ASSIGN => '=',
            Tokens::SMALLER=>'<',
            Tokens::EQUAL => '_',
            Tokens::GREATER=>'>',//<
            //Tokens::PRINT=>'p',
            _t => return lhs, //fatal!("o non (op): {:?}", t), //bad token
        };
        if let Some(infix) = infix_binding_power(op) {
            let (l_bp, r_bp) = infix;
            if l_bp < min_power {
                break;
            }

            v.next();
            let rhs = parse(v, r_bp);

            lhs = ParseResult::Cons(d, vec![lhs, rhs]); //op
        }
    }
    //println!("PARSE EXIT");
    lhs
}
pub fn back_to_tokens(b: ParseResult,in_if:bool) -> (Vec<Token>, Type) {
    let mut k = Vec::new();
    let mut itype = Type::ANY;

    match b {
        ParseResult::Atom(token, d) => {
            //T
            k.push(token); //O//T
            itype = d;
        }
        ParseResult::Cons(token, vec) => {
            if token.token == Tokens::ASSIGN && !in_if {
                if let Some(d) = vec.get(1 - 1) {
                    if let ParseResult::Atom(token, tp) = d {
                        //_
                        if token.token == Tokens::VARIABLE {
                            let mut m = token.clone();
                            m.token = Tokens::VARIABLE_ASSIGN;
                            k.push(m);
                        } else {
                            error_parser(
                                token.start,
                                token.line as usize,
                                "Le coté gauche d'un 'vaut' doit être une variable".to_string(),
                            ); //,tp
                        }
                    } else if let ParseResult::Cons(a, b) = d {
                        //,tp
                        error_parser(
                            a.start,
                            a.line as usize,
                            "Le coté gauche d'un 'vaut' ne peut pas être une expression"
                                .to_string(),
                        );
                    }
                }
                let mut d = vec.into_iter();
                d.next();
                // let c=d.collect();
                //k.append(c);
                for i in d {
                    k.append(&mut back_to_tokens(i,in_if).0);
                }
                k.push(token);
                return (k, Type::ANY); //itype
            }
            if token.token==Tokens::LOOP{//IF
                let mut n=token.clone();
                n.token=Tokens::TOKENS_IF_BEFORE;
                k.push(n);
                let d = vec.into_iter();//mut 
                //d.next();
                // let c=d.collect();
                //k.append(c);
                for i in d {//in_if
                    k.append(&mut back_to_tokens(i,true).0);
                }
                k.push(token);
                return (k,Type::ANY);
            }
            //V//T
            for i in vec {
                //V//in_if
                let t = &mut back_to_tokens(i,token.token==Tokens::IF||token.token==Tokens::LOOP||in_if); //.0
                match t.1 {
                    //itype
                    Type::BOOL => {
                        if itype == Type::STR && token.token == Tokens::ADD { //}
                             //  itype=Type::BOOL;
                        } else if token.token != Tokens::EQUAL
                            && token.token != Tokens::IF
                            && token.token != Tokens::LOOP
                            &&token.token!=Tokens::ASSIGN
                        {
                            //u texte// (sauf l'addition)//D
                           // println!("{:?}",token.token);
                            error_parser(token.start,token.line as usize,"Il est impossible de faire des opérations mathématiques sur des oui ou des non".to_string());
                        }
                    }
                    Type::NUM => {
                        if itype != Type::BOOL {
                            if itype == Type::STR && token.token != Tokens::ADD {
                                error_parser(token.start,token.line as usize,"Il est impossible de faire des opérations mathématiques sur du texte (sauf l'addition)".to_string());
                            }
                            // !
                            else {
                                //}
                                itype = Type::NUM;
                            }
                        } else {
                        }
                    }
                    Type::STR => {
                        //BOOL
                        if is_op(&token.token) && token.token != Tokens::ADD {
                            error_parser(token.start,token.line as usize,"Il est impossible de faire des opérations mathématiques sur du texte (sauf l'addition)".to_string());
                        } else {
                            itype = Type::STR;
                        }
                    }
                    _ => {}
                }
                k.append(&mut t.0); //b//push
            }

            k.push(token); //T
        }
    }
    if &k.clone().into_iter().last().unwrap().token==&Tokens::IF{
        //let mut n=0;
        for i in &mut k{
            if i.token==Tokens::ASSIGN{
                i.token=Tokens::EQUAL;
            }
            //n+=1;
        }
    }
    (k, itype)
}

pub fn dis(dvec: &Vec<Token>) -> String {
    let mut d = String::new();
    d.push_str("Sortie du parser:\n");
    for i in dvec {
        //dis//&//|
        d.push_str(format!("  {:?} \t {}\n", i.token, i.snipet).as_str());
    }
    d.push_str(format!("Quantité de tokens: {}", dvec.len()).as_str()); //i
    d
}
pub fn is_op(d: &Tokens) -> bool {
    match d {
        Tokens::DIV => true,
        Tokens::MOD => true,
        Tokens::MUL => true,
        Tokens::ADD => true,
        Tokens::SUB => true,
        _ => false,
    }
}
pub fn error_parser(char_index: usize, line_number: usize, message: String) {
    //str//d
    let text = lexer::FILE.lock().unwrap(); //get_mut().unwrap();//k//message//text//into_inner()//&
    println!("\nErreur!");
    println!("{}", message);
    if let Some(d) = &Some(text) {
        //n//self.text
        let n = d.split("\n"); //pat//line
        let r = (char_index, line_number); //self.instructions_origins.get(self.instruction).unwrap();
        let mut it = 0;
        let mut ofs = 0;
        for i in n {
            if it == r.1 {
                println!("{}", i);
                for _i in 0..(r.0 - ofs) {
                    ////-1-1
                    print!(" ");
                }
                println!(" {} (ligne {})", "^ici", r.1 + 1); //{:0>8}//5//'//:0>////i
            }
            ofs += i.len();
            it += 1;
        }
    }
    if ERROR_INSTANTLY_QUITS {
        fatal!();
    }
}
pub const ERROR_INSTANTLY_QUITS: bool = !false; //error_instantly_quitserror_instantly_quits//!
pub const DEBUG_PARSER: bool = false;
pub const VAUT_DANS_LES_SI_EQUIVAUT_A_EGAL:bool=true;
pub fn is_bool(d: String) -> bool {
    match d.as_str() {
        "Oui" | "oui" => true,
        "Non" | "non" => true,
        _ => false,
    }
}
