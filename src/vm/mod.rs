use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
   // thread,
    time:: SystemTime,//Duration,//{//}
}; //, ops::IndexMut//{//}
mod instruction_debugger;

use rmp_serde::decode::Error;
use serde::{Deserialize, Serialize};
//use rmp::{Deserializer, Serializer};//s
use crate::fatal;
//use either::Either;
const VM_DEBUG_LAYER: bool = false; // !//!//!//!!!//!//!//!//!//!

pub struct MachineVirtuelle {
    pub instruction: usize,
    pub instructions: Vec<Chunk>,
    pub instructions_origins: Vec<(usize, usize)>,
    pub stack: Vec<ValueContainer>,
    pub text: Option<String>,
    pub depth: u32, //String//usize
    pub var_map: Vec<HashMap<String, ValueContainer>>,
    pub time: u32, //i32
}

impl MachineVirtuelle {
    pub fn serialize(&mut self) -> String {
        let m = rmp_serde::to_vec(&self.instructions); //val
        if let Ok(result) = m {
            return unsafe { String::from_utf8_unchecked(result) };
        } else if let Err(d) = m {
            fatal!("Erreur de sérialization\n {}", d) //;
        }
        //else{
        fatal!()
        // }
        //String::new()
    }

    pub fn deserialize(d: Vec<u8>) -> MachineVirtuelle {
        //String
        let mut mv = MachineVirtuelle::new(None);
        let k: Result<Vec<Chunk>, Error> = rmp_serde::from_slice(&d); //read(rd)//&selffrom_u8////String::as_bytes(&d)
        if let Ok(result) = k {
            mv.instructions = result; //chun
        } else if let Err(error) = k {
            fatal!("Le programme chargé n'a pas pu être désérializé\n{}", error);
        }
        mv
    }

    pub fn new(str: Option<String>) -> MachineVirtuelle {
        //()//()
        MachineVirtuelle {
            instruction: 0,
            instructions: Vec::new(),
            instructions_origins: Vec::new(),
            stack: Vec::new(),
            text: str, //Option<String>,
            depth: 0,
            var_map: vec![HashMap::new()], //;
            time: 0,                       //u32
        }
    }

    pub fn chunk(&mut self, chunk: Chunk, start: usize, line: usize) {
        self.instructions.push(chunk); //chu
        self.instructions_origins.push((start, line));
    }

    pub fn error(&mut self, message: String) -> ! {
        //let n=;
        println!("\nErreur!");
        println!("{}", message);

        if let Some(d) = &self.text {
            //n
            let n = d.split("\n"); //pat
            let r = self.instructions_origins.get(self.instruction).unwrap();
            let mut it = 0;
            let mut ofs = 0;
            for i in n {
                if it == r.1 {
                    println!("{}", i);
                    for _i in 0..(r.0 - ofs - 1 - 1) {
                        //
                        print!(" ");
                    }
                    println!(" {} (ligne {})", "^ici", r.1 + 1); //{:0>8}//5//'//:0>////i
                } //value, index
                ofs += i.len();
                it += 1;
            }
        }
        println!("Pour des messages d'erreur plus en profondeur, exécutez \n\tverbose -c -d erreur=oui <fichier>");
        fatal!()
    }

    pub fn start(&mut self, print_instructions: bool) {
        if self.instructions.is_empty() {
            return;
        }

        if print_instructions {
            instruction_debugger::print_instructions(&self.instructions);
        }

        //const VM_DEBUG_LAYER: bool = ;
        if VM_DEBUG_LAYER {
            let mut n = 0;
            for i in &self.instructions {
                println!(
                    "{}\t {:?} \tLigne: {}",
                    n,
                    i,
                    self.instructions_origins.get(n).unwrap().1 + 1
                ); // //0
                n += 1;
            }
        }
        self.time = 0;
        //self.start_time=

        let start = SystemTime::now();
        //let since_the_epoch = start
        //.duration_since(UNIX_EPOCH)
        //.expect("Time went backwards");
        // let mut start_time=since_the_epoch;
        //   let mut current_time=since_the_epoch;
        self.instruction = 0;
        self.depth = 0; //u32;
        let mut r = self.instructions.get(0).unwrap();
        let mut instruction_move;// = 0;
        let mut should_instruction_move;// = false;

        while r != &Chunk::EOF {
            //println!("Instruction: {:?} {:?}",r,self.var_map);
            instruction_move = 0; //let mut
            should_instruction_move = false; //let mut
                                             //println!("{:?}",self.stack);
                                             // /  println!("Parsing instruction {:?}, stack is {:?}",r,self.stack);
                                             // thread::sleep(Duration::from_millis(100*4*4));//500
                                             //println!("{}",self.stack.len());
            match r.clone() {
                Chunk::ARRAY_BEGIN => {
                    self.spawn(ValueContainer::new_marker(0));
                }
                Chunk::ARRAY_END => {
                    let mut v = Vec::new();
                    let mut k = self.stack.pop();
                    while k.is_some() && (&k.clone().unwrap().value != &Value::MARKER) {
                        v.push(k.unwrap());

                        k = self.stack.pop(); //.
                    }

                    v.reverse();
                    self.spawn(ValueContainer::new_array(v, 0));
                }
                Chunk::CLOCK => {
                    let d = SystemTime::now()
                        .duration_since(start)
                        .expect("Le temps a reculé"); //earlier//msg
                    let k = d.as_millis() as i32;
                    self.spawn(ValueContainer::new_num(k, 0)); //self.stack.push()
                }
                Chunk::VARIABLEREF(dddd) => {
                    //println!("REF");
                    //  println!("{:?}",self.var_map);
                    let mut i;// = 0;
                    let mut has_value = false;
                    for j in 0..self.var_map.len() {
                        i = self.var_map.len() - j - 1; //i
                        if let Some(d) = self.var_map.get(i) {
                            //j
                            //println!("R");
                            if let Some(r) = d.get(&dddd) {
                                has_value = true;
                                self.spawn(r.clone());
                                break;
                            }
                        }
                    } //e lui assigner une
                    if !has_value {
                        self.error(format!(
                            "La variable {} n'existe pas. Impossible d'obtenir sa valeur.\n{:?}",
                            dddd, self.var_map
                        ));
                    }
                    //for i in self.var_map.reverse(){

                    // }
                }
                Chunk::VARIABLEASS(dddd) => {
                    //string
                    self.spawn(ValueContainer::new_string(dddd, 0)); // { value: (), index: () }//value, index
                }
                Chunk::GREATER => {
                    let a = self.pop(); //>//<//c//c
                    let b = self.pop();
                    //println!("GREATER {}>{}",a.i32_val(),b.i32_val());
                    self.spawn(ValueContainer::new_bool(b.i32_val() > a.i32_val(), 0));
                    //d
                } //()
                Chunk::SMALLER => {
                    //GREATER
                    let a = self.pop(); //>//<//c//c
                    let b = self.pop();
                    //>
                    //println!("SMALLER {}>{}",a.i32_val(),b.i32_val());
                    self.spawn(ValueContainer::new_bool(b.i32_val() < a.i32_val(), 0));
                    //d
                } //()
                Chunk::ASSIGN => {
                    //        println!(
                    //      "{:?}",&self.var_map.len()
                    //           );
                    let value = self.pop();

                    let index = self.pop();

                    //println!("{}",index.str());//value
                    if let Value::STRING(_d) = &index.value { //D
                         //STRING//INT
                    } else {
                        self.error("Erreur! Le nom de la variable est manquant!".to_string());
                    }
                    let mut has_assigned = false;
                    for i in &mut self.var_map {
                        if i.contains_key(&(index.str())) {
                            //println!("FIRST");
                            //k//str()
                            //i[&index.str()]=value;
                            {
                                let c = i.get(&index.str());
                                if !c.unwrap().same_type_as(&value) {
                                    //val
                                    self.error(format!("Erreur! La variable {} n'a pas le même type que la variable qu'on lui assigne!",index.i32_val()));
                                    //_
                                }
                            }
                            i.insert(index.str(), value.clone()); //&//str()
                            has_assigned = true;
                            break;
                        }
                    }
                    let m = self.var_map.len() - 1;
                    //println!("{}",self.var_map.len());
                    if !has_assigned {
                        //println!("NOUVELLE VARIABLE");
                        let d = self.var_map.get_mut(m);
                        if let Some(d) = d {
                            // d[&index.str()]=value;
                            // println!("{} {}",index.str(),m);
                            d.insert(index.str(), value); //&//i//.str()
                            has_assigned = true;
                            //break;
                        }
                    }
                    if !has_assigned {
                        self.error(format!("Cette erreur est un problème de la machine virtuelle (pas du code). Les hashmaps sont manquants!"));
                    }
                    //println!("{:?}",self.var_map);
                }
                Chunk::IGNORE => {}
                Chunk::CONST(d) => {
                    //println!("CONST {:?}",d);
                    self.spawn(d.clone()); //d
                }
                Chunk::DIV => {
                    let b = self.pop(); //self.pop()
                    let a = self.pop(); //m
                    self.spawn(a.div(b)); //add
                }
                Chunk::MOD => {
                    let b = self.pop();
                    let a = self.pop();
                    self.spawn(a.modulo(b));
                }
                Chunk::ADD => {
                    //DIV
                    let b = self.pop(); //self.pop()
                    let a = self.pop(); //m

                    self.spawn(a.add(b)); //add//div
                }
                Chunk::END => {
                    //println!("POP!");
                    self.var_map.pop();
                    self.depth -= 1;
                }
                Chunk::JMP(b) => {
                    //let m=true;
                    //if !m{//&&false
                    should_instruction_move = true;
                    instruction_move = b;
                    //}
                }
                Chunk::JMPEND(b) => {
                    //println!("JMPEND");
                    self.var_map.pop();
                    self.depth -= 1;
                    should_instruction_move = true;
                    instruction_move = b;
                }
                Chunk::JMPIFFALSE(b) => {
                    //println!("{:?}",self.stack);
                    //println!("DEEP");
                    self.var_map.push(HashMap::new()); // println!("R");
                    self.depth += 1;
                    let m = self.pop();
                    //println!("IF {:?}",m);
                    let m = m.is_true(); //.clone();

                    /*if !m{//!m.is_true()
                        should_instruction_move=true;
                        //self.instruction=b;/ / *
                        instruction_move=(b.clone());// *
                        //continue;
                    }  */
                    if !m {
                        //&&false
                        should_instruction_move = true;
                        instruction_move = b;
                    }
                }
                Chunk::MUL => {
                    //DIV
                    let a = self.pop(); //m
                    let b = self.pop(); //self.pop()
                    self.spawn(a.mul(b)); //add//div
                }
                Chunk::SUB => {
                    //DIV

                    let b = self.pop(); //self.pop()
                    let a = self.pop(); //m
                    self.spawn(a.sub(b)); //add//div
                }
                Chunk::PRINT => {
                    println!("{}", self.pop().str()); //stack
                }
                Chunk::EQUAL => {
                    let a = self.pop();
                    let b = self.pop(); //value
                    self.spawn(ValueContainer::new_bool(a == b, self.depth)); //index
                }
                Chunk::GREATER_EQ => {
                    let a = self.pop();
                    let b = self.pop();
                    self.spawn(ValueContainer::new_bool(b.i32_val() >= a.i32_val(), 0));
                }
                Chunk::SMALLER_EQ => {
                    let a = self.pop();
                    let b = self.pop();
                    self.spawn(ValueContainer::new_bool(b.i32_val() <= a.i32_val(), 0));
                }
                Chunk::EOF => {}
            }
            if should_instruction_move {
                self.instruction = instruction_move;
                //println!("{}",self.instruction);
            } else {
                self.instruction += 1; //index
            }
            if let Some(d) = self.instructions.get(self.instruction) {
                r = d;
            } else {
                break;
            }
            //r = ;//.unwrap(); //index

            //self.peek
        }
        //println!("{:?}",self.stack);
    }
    pub fn pop(&mut self) -> ValueContainer {
        if let Some(d) = self.stack.pop() {
            d
        } else {
            //fatal!
            self.error("Le stack est vide. il est donc impossible de \"pop\"".to_string())
            //;
        }
        //return self.stack.pop();
    }
    pub fn set_instruction(&mut self, index: usize, d: Chunk) {
        //println!("{:?} ->,{:?}",self.instructions[index],&d);//////
        self.instructions[index] = d;
    }
    pub fn append(&mut self, d: ValueContainer) {
        self.stack.push(d); //other//append
    }
    pub fn spawn(&mut self, d: ValueContainer) {
        //_str//new//str:String
        let mut c = d;
        c.index = self.depth; //;
        self.append(c); //push(c)//.clone()//stack.
    }
    //pub fn update()
    pub fn clean_depth(&mut self) {
        let mut to_delete = 0; //Vec::new();
        for i in 0..self.stack.len() {
            let index = self.stack.len() - i;
            if let Some(d) = self.stack.get(index) {
                if d.index > self.depth {
                    to_delete += 1; //.push(index);
                } else {
                    break;
                }
            }
        }
        for _i in 0..to_delete {
            self.stack.pop(); //()
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)] //Copy,
pub enum Chunk {
    //struct
    ARRAY_BEGIN,
    ARRAY_END,
    CLOCK,
    CONST(ValueContainer),
    ADD,
    SUB,
    DIV,
    MOD,
    MUL,
    SMALLER,
    GREATER,    //B
    GREATER_EQ, //B
    SMALLER_EQ, //B
    EQUAL,
    EOF,
    PRINT,               //String//usize//usize
    VARIABLEREF(String), //ValueContainer//_//String
    VARIABLEASS(String), //_ASSIGN
    ASSIGN,              //i32//i32
    JMPIFFALSE(usize),   //u16
    JMPEND(usize),       //_//_
    JMP(usize),          //4
    END,
    IGNORE,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    INT(i32),
    STRING(String),
    BOOL(bool),
    FLOAT(f32),
    ARRAY(Vec<ValueContainer>),
    MARKER,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)] //,Copy//,IndexMut
pub struct ValueContainer {
    //Box<//>
    pub value: Value, //Either<Either<String, bool>, i32>, //f

    pub index: u32, //pub value:
}

impl ValueContainer {
    pub fn new_array(v: Vec<ValueContainer>, index: u32) -> ValueContainer {
        ValueContainer {
            value: Value::ARRAY(v),
            index: index,
        }
    }
    pub fn new_marker(index: u32) -> ValueContainer {
        //+//N
        ValueContainer {
            value: Value::MARKER,
            index: index,
        } //&//O
    }
    pub fn same_type_as(&self, other: &ValueContainer) -> bool {
        //d
        return self.type_i32() == other.type_i32();
    }
    pub fn type_i32(&self) -> i32 {
        match self.value {
            Value::INT(_) => 0,
            Value::BOOL(_) => 1,
            Value::STRING(_) => 2,
            Value::FLOAT(_) => 3,
            Value::MARKER => 4, //(_)
            Value::ARRAY(_) => 5,
        }
    }
    pub fn new_string(value: String, index: u32) -> ValueContainer {
        ValueContainer {
            value: Value::STRING(value), //Either::Left(Either::Left(value)),
            index,
        } //()
    }
    pub fn new_float(value: f32, index: u32) -> ValueContainer {
        ValueContainer {
            value: Value::FLOAT(value),
            index,
        } //()//()
    }
    pub fn div(mut self, b: ValueContainer) -> ValueContainer {
        let a = self.i32_val(); //r//b
        let b = b.i32_val(); //: ValueContainer
        self.value = Value::INT(a / b);
        self
    }
    pub fn modulo(mut self, b: ValueContainer) -> ValueContainer {
        let a = self.i32_val();
        let b = b.i32_val();
        self.value = Value::INT(a % b);
        self
    }
    pub fn sub(mut self, b: ValueContainer) -> ValueContainer {
        //div
        let a = self.i32_val(); //r//b
        let b = b.i32_val(); //: ValueContainer
        self.value = Value::INT(a - b);
        // /
        self
    }
    pub fn mul(mut self, b: ValueContainer) -> ValueContainer {
        //sub
        //div
        let a = self.i32_val(); //r//b
        let b = b.i32_val(); //: ValueContainer
        self.value = Value::INT(a * b); //-
                                        //
        self
    }
    pub fn str(&self) -> String {
        match self.value.clone() {
            Value::STRING(d) => d,
            Value::BOOL(d) => {
                if d {
                    String::from("Oui") //;
                } else {
                    String::from("Non") //';
                }
            }
            Value::MARKER => {
                fatal!("Ceci est une erreur du compileur (ou votre code est tarabiscoté). Impossible de convertir des types 'Marqueur' en texte");
            }
            Value::ARRAY(v) => {
                let mut t = "".to_owned(); //("");//
                for i in v {
                    if !t.is_empty() {
                        t.push_str(&", ");
                    } //v
                    t = t + &i.str(); //.push(v);
                }
                t
            }
            Value::FLOAT(d) => {
                //num
                return f32::to_string(&d);
            }
            Value::INT(d) => {
                i32::to_string(&d) //String::from//&self
            }
        }
    }
    pub fn add(mut self, b: ValueContainer) -> ValueContainer {
        match self.value {
            Value::STRING(d) => {
                let m = b.str();
                self.value = Value::STRING(d + &m);
                self
            }
            _ => {
                if let Value::STRING(d) = b.value.clone() {
                    //&
                    return ValueContainer::new_string(self.str() + &d, self.index);
                    //+//&//b
                    //return b.add(self);//*//tring
                }
                //println!("ok");
                let a = self.i32_val(); //r//b
                let b = b.i32_val(); //: ValueContainer
                self.value = Value::INT(a + b); //-
                                                // /
                self
            } //Value::BOOL(c)=>{//d//_

              //}
        }
    }

    pub fn i32_val(&self) -> i32 {
        match &self.value {
            Value::INT(_d) => {
                return *_d;
            } //todo!(),
            Value::STRING(_) => {
                fatal!("Il est impossible de convertir du texte en nombre");
            } // todo!(),
            Value::BOOL(_) => {
                fatal!("Il est impossible de convertir des valeurs Oui/Non en nombre");
            } //todo!(),
            Value::FLOAT(_d) => {
                let m = _d.floor() as i32;
                m
            }
            Value::MARKER => {
                fatal!("Il est impossible de convertir des marqueurs en nombre");
            }
            Value::ARRAY(_d) => {
                fatal!("Il est impossible de convertir des listes en nombre");
            }
        }
    }

    pub fn new_bool(value: bool, index: u32) -> ValueContainer {
        //bool
        ValueContainer {
            //bool
            value: Value::BOOL(value), //Either::Left(Either::Right(value)),
            index: index,
        } //()
    }

    pub fn new_num(value: i32, index: u32) -> ValueContainer {
        //()
        ValueContainer {
            //f
            value: Value::INT(value), //Either::Right(value),//i32
            index: index,
        } //()
    }

    pub fn is_true(&self) -> bool {
        match &self.value {
            Value::INT(d) => {
                //Either::Right
                return d != &0; //=//BOOL
            }
            Value::FLOAT(d) => {
                return d < &0. && d > &0.;
            }
            //Either::Left(a) => {
            //match &a {
            Value::STRING(a) => {
                //Either::Left
                return !a.is_empty();
            }
            Value::BOOL(d) => {
                //Either::Right
                //_t
                return *d;
            } //}
            Value::MARKER => {
                println!("Quelque chose ne tourne pas rond dans votre code. Vérifiez vos déclarations de listes");
                true
            }
            Value::ARRAY(d) => return !d.is_empty(), //}
        }
    }
}

#[derive(PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum ValueType {
    //struct
    BOOL,
    STRING,
    NUM,
}

#[allow(dead_code)]
fn input_2(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}
