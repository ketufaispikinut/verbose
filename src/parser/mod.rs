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
    pub fn fail(&self,t:String){
        
        fatal!("{}", t);//message
    }
    pub fn is_at_end(&self) -> bool {
        return self.index >= self.vec.len();
    }
    pub fn consume(&mut self, token: Tokens, message: &str) {
        if self.peek().token == token {
            self.next();
        } else {
            println!("Ligne: {}",self.vec[self.index].line+1);
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
        while is_op(&k.peek().token) {
            let n = parse(&mut k, 0);
            if DEBUG_TREE {
                debug_tree(&n); //&b//i
                println!("");
            }
            let mut c = back_to_tokens(n, false); //

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

pub fn is_op(t: &Tokens) -> bool {
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
        | Tokens::SMALLER_EQ
        | Tokens::GREATER_EQ
        | Tokens::EQUAL
        | Tokens::PRINT
        | Tokens::IF//,
        | Tokens::LOOP//,
        | Tokens::INDEX
        | Tokens::ADD_L
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

#[derive(Debug,Clone)]
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
            match is_bool(f.snippet.clone()) {
                true => Type::BOOL,
                _ => Type::NUM,
            },
        ),
        Tokens::CONST_STR => ParseResult::Atom(f, Type::STR),
        Tokens::VARIABLE => ParseResult::Atom(f, Type::ANY),
        Tokens::CLOCK => ParseResult::Atom(f, Type::NUM),
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
        //Tokens::ADD_R=>{
       //     println!("DDDD");
       //     fatal!("")
      //  }
        Tokens::ADD_L=>{
            //println!("OOOOO");
            let m=parse(v,0);
            //println!("m: {:?}",m);
            v.consume(Tokens::ADD_R, "Syntaxe: Ajoute X à VARIABLE");//message//let c=
            let b=parse(v,0);
            //println!("b: {:?}",b);
            if let ParseResult::Atom(a,_b )=&b{//c
                if a.token!=Tokens::VARIABLE{//&
                    v.fail("Syntaxe: Ajoute X à VARIABLE".to_string());//v.consume(Tokens::, message)
                }
            }
            //println!("{:?}",v.peek());
            //println!("OK DOK");
            return ParseResult::Cons(f,vec![m,b]);
        }
        Tokens::L_ARRAY => {
            //println!("ARRAY!");
            let mut lhs = Vec::new();
            let mut l = f.clone();
            l.token = Tokens::R_ARRAY; //()
            
            lhs.push(ParseResult::Atom(l, Type::ANY)); //()
            if v.peek().token==Tokens::R_ARRAY{//d
                return ParseResult::Cons(f,lhs);//l//Vec::new()
            }
            lhs.push(parse(v, 0)); //lexer
                                   //assert_eq!(lexer.next(), Token::Op(')'));
            let mut d = v.peek();
            
            //println!("ARBEGIN");
            loop {
                //while true
                // println!("STEP {:?}",d);
                match d.token {
                    Tokens::COMMA => {
                        // println!("COMMA");
                        v.next();
                        lhs.push(parse(v, 0));

                        // v.next();
                    }
                    Tokens::R_ARRAY => {
                        v.next();
                        //println!("AREND");
                        //   println!("FIN DE L'ARRAY");
                        //lhs.push();
                        break;
                    }
                    _ => {
                        //dbg!(d).to_owned()
                        error_parser(f.start, f.line as usize, format!("{} {:?}",String::from("Une virgule ou un ] est attendu pour finir une liste. Obtenu: "),d));
                    }
                }
                //v.next();
                d = v.peek();
            }
            //  v.consume(
            //      Tokens::PARDRO,
            //      &"Une parenthèse fermante est ici nécessaire",
            //  ); //lexer
            //lhs //return lhs;
            return ParseResult::Cons(f, lhs);
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
        Tokens::POP=>{//0
            let lhs = parse(v, min_power*0); //lexer
            return ParseResult::Cons(f,vec![lhs]); //return lhs;
        }
        t_ => {
            fatal!("o non (lhs): {:?} à la ligne {}", t_,f.line+1);
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
            Tokens::SMALLER => '<',
            Tokens::SMALLER_EQ => '≤',
            Tokens::GREATER_EQ => '≥',
            Tokens::EQUAL => '_',
            Tokens::GREATER => '>', //<
            Tokens::INDEX=>'.',
            Tokens::POP=>'P',
            //Tokens::ADD_R=>'.',
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
pub fn debug_tree(b: &ParseResult) {
    match b {
        ParseResult::Atom(token, d) => {
            print!("{}", token.snippet);
        }
        ParseResult::Cons(tokens, vec) => {
            print!(" {{ "); //\
            print!("{} -> (", tokens.snippet); //:?//:?//
                                               //pri

            for i in vec {
                print!(" ");
                debug_tree(i); //b
            }
            print!(")");
            print!(" }} ");
        }
    }
}
pub fn back_to_tokens(b: ParseResult, in_if: bool) -> (Vec<Token>, Type) {
    let mut k = Vec::new();
    let mut itype = Type::ANY;

    match b {
        ParseResult::Atom(token, d) => {
            //T
            k.push(token); //O//T
            itype = d;
        }
        ParseResult::Cons(token, vec) => {
            if token.token==Tokens::POP{
                let m=vec.get(0);//pop()
                if vec.len()!=1{//0

                    
                }
                else{
                    let d=m.unwrap();//mut 
                    match d{
                        ParseResult::Atom(a,_b)=>{
                            let mut a=a.clone();
                            if a.token==Tokens::VARIABLE{
                            a.token=Tokens::VARIABLE_ASSIGN;
                            return (vec![a,token.clone()],Type::ANY);//.clone()
                        }
                        }//back_to_tokens(b, in_if)
                        _=>{

                        }
                       // else{

                       // }
                }//return 
                fatal!("Ligne: {}\n Syntaxe: enlève le dernier élément de VARIABLE",token.line+1);

            }
        }
            else if token.token == Tokens::ASSIGN && !in_if {
                if let Some(d) = vec.get(1 - 1) {
                    if let ParseResult::Atom(token, tp) = d {
                        //_
                        if token.token == Tokens::VARIABLE {
                            let mut m = token.clone();
                            m.token = Tokens::VARIABLE_ASSIGN;
                            k.push(m);
                         } 
                        else {
                            error_parser(
                                token.start,
                                token.line as usize,
                                "Le coté gauche d'un 'vaut' doit être une variable".to_string(),
                            ); //,tp
                        }
                    } else if let ParseResult::Cons(a, b) = d {
                        if a.token==Tokens::INDEX{//else if token
                            //println!("{:?}",b);
                            //let mut c=b.clone();
                            //c.reverse();
                            // /for i in c{//b//i.clone()
                                //k.push(i.b);//m
                            //}
                            let mut c=b.clone();
                            let mut d:Vec<Token>=Vec::new();
                            c.reverse();
                            for i in c{//b
                                
                                d.append(&mut back_to_tokens(i.clone(), in_if).0);//other
                            }//c
                            for i in &mut d{
                                if i.token==Tokens::VARIABLE{
                                    i.token=Tokens::VARIABLE_ASSIGN;
                                    //println!("ASS");
                                }
                            }
                            k.append(&mut d);//c
                            //return (d,Type::ANY);
                            
                        } 
                        else{
                        //,tp
                        error_parser(
                            a.start,
                            a.line as usize,
                            "Le coté gauche d'un 'vaut' ne peut pas être une expression"
                                .to_string(),
                        );
                    }
                    }
                }
                let mut d = vec.into_iter();
                d.next();
                // let c=d.collect();
                //k.append(c);
                for i in d {
                    k.append(&mut back_to_tokens(i, in_if).0);
                }
                k.push(token);
                return (k, Type::ANY); //itype
            }
            else if token.token==Tokens::ADD_L{
                let mut v=vec.clone();
                let mut m=v.pop().unwrap();//vec
                if let ParseResult::Atom(d,_a )=&mut m{
                    d.token=Tokens::CONST_STR;//VARIABLE_ASSIGN;
                   //println!("ASS");
                }

                v.push(m);
                for i in v{//b//ec
                    k.append(&mut back_to_tokens(i, in_if).0);
                }
                k.push(token);
                return (k,Type::ANY)
            }
            if token.token == Tokens::LOOP {
                //IF
                let mut n = token.clone();
                n.token = Tokens::TOKENS_IF_BEFORE;
                k.push(n);
                let d = vec.into_iter(); //mut
                                         //d.next();
                                         // let c=d.collect();
                                         //k.append(c);
                for i in d {
                    //in_if
                    k.append(&mut back_to_tokens(i, true).0);
                }
                k.push(token);
                return (k, Type::ANY);
            }
            //V//T
            for i in vec {
                //V//in_if
                let t = &mut back_to_tokens(
                    i,
                    token.token == Tokens::IF || token.token == Tokens::LOOP || in_if,
                ); //.0
                match t.1 {
                    //itype
                    Type::BOOL => {
                        if itype == Type::STR && token.token == Tokens::ADD { //}
                             //  itype=Type::BOOL;
                        } else if token.token != Tokens::EQUAL
                            && token.token != Tokens::IF
                            && token.token != Tokens::LOOP
                            && token.token != Tokens::ASSIGN
                        {
                            //u texte// (sauf l'addition)//D
                            // println!("{:?}",token.token);
                            error_parser(token.start, token.line as usize,"Il est impossible de faire des opérations mathématiques sur des oui ou des non".to_string());
                        }
                    }
                    Type::NUM => {
                        if itype != Type::BOOL {
                            if itype == Type::STR
                                && token.token != Tokens::ADD
                                && token.token != Tokens::EQUAL
                                && token.token != Tokens::ASSIGN
                            {
                                error_parser(token.start, token.line as usize, "Il est impossible de faire des opérations mathématiques sur du texte (sauf l'addition)".to_string());
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
                        if is_op(&token.token)
                            && token.token != Tokens::ADD
                            && !token_supports_str(&token.token)
                        {
                            //bool//D
                            error_parser(token.start, token.line as usize, "Il est impossible de faire des opérations mathématiques sur du texte (sauf l'addition)".to_string());
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
    if &k.clone().into_iter().last().unwrap().token == &Tokens::IF {
        //let mut n=0;
        for i in &mut k {
            if i.token == Tokens::ASSIGN {
                i.token = Tokens::EQUAL;
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
        d.push_str(format!("  {:?} \t {}\n", i.token, i.snippet).as_str());
    }
    d.push_str(format!("Quantité de tokens: {}", dvec.len()).as_str()); //i
    d
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
pub const ERROR_INSTANTLY_QUITS: bool = true; //error_instantly_quitserror_instantly_quits//!
pub const DEBUG_PARSER: bool = false;// !
pub const VAUT_DANS_LES_SI_EQUIVAUT_A_EGAL: bool = true;
pub fn is_bool(d: String) -> bool {
    match d.as_str() {
        "Oui" | "oui" => true,
        "Non" | "non" => true,
        _ => false,
    }
}

pub const DEBUG_TREE: bool = !true;
pub fn token_supports_str(d: &Tokens) -> bool {
    match d {
        Tokens::ASSIGN => true,
        Tokens::EQUAL => true,
        Tokens::ADD => true,
        Tokens::PRINT => true,
        _ => {
            //println!("{:?}",d);
            false
        }
    }
}
