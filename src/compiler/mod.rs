use std::collections::HashMap;

use crate::{
    fatal,
    lexer::Token,
    lexer::{
        self,
        Tokens::{self, *},
    },
    vm::{Chunk, MachineVirtuelle, ValueContainer},
};

#[derive(Debug)]
pub struct JmpBackwardRef {
    //index: usize,
    index_to: usize,
    is_loop: bool,
    usize_from: usize,
    push_bool: bool,
}

pub fn compile_to_bitcode(tokens: &Vec<Token>, vm: &mut MachineVirtuelle) {
    //t
    let mut n = 0;
    let mut backward_jmps: Vec<JmpBackwardRef> = Vec::new();
    let mut skip = false;
    let mut hashmap_vars: HashMap<String, i32> = HashMap::new(); //ne(&self, other)
    let mut var_count = 0; //d=self.
    let mut if_index = Vec::new();
    for i in tokens {
        // println!("TK");
        {
            if skip {
                skip = false;
                n += 1;
                continue;
            }
            let mut m: usize = 0;
            let mut vec = Vec::new();

            // let mut sk=false;
            //println!("{:?}",backward_jmps);
            for j in &backward_jmps {
                if j.index_to == n {
                    //println!("{:?}",i.token);//16

                    //println!("ENDME0");

                    if j.is_loop {
                        //()
                        let n = if_index.pop().expect(
                            format!("Ceci est une erreur du compileur! \n {:?}", &tokens).as_str(),
                        ); //msg//e
                           //j.usize_from - 1

                        vm.chunk(Chunk::JMPEND(n), i.start, i.line as usize);
                        vm.chunk(Chunk::IGNORE, i.start, i.line as usize);
                        //println!("CD");
                        vm.set_instruction(
                            //+1//index_to//.index
                            j.usize_from, //**//- 1 + 1-1+1+1
                            Chunk::JMPIFFALSE(vm.instructions.len() as usize),
                        ); //()//-1
                    } else {
                        vm.chunk(Chunk::END, i.start, i.line as usize); //chunk//IFEND//IGNORE
                        vm.chunk(Chunk::IGNORE, i.start, i.line as usize);
                        // println!("CE");
                        vm.set_instruction(
                            //.index
                            j.usize_from, //**//- 1 + 1-1+1+1//_to
                            Chunk::JMPIFFALSE(vm.instructions.len() as usize),
                        ); //()//-1
                        if j.push_bool {
                            vm.set_instruction(
                                j.usize_from + 1,
                                Chunk::CONST(ValueContainer {
                                    value: crate::vm::Value::BOOL(false),
                                    index: 0,
                                }),
                            )
                        }
                    }
                    vec.push(m);
                    break;
                    //   sk=true;
                }
                m += 1;

                //else{
                //    println!("{} {}",j.index_to,n);
                //}
            }
            // if sk{
            //     continue;
            // }
            vec.reverse(); //serve(additional)//
            for i in vec {
                //
                backward_jmps.remove(i);
            } //
              //
        }
        match i.token {
            CLOCK => {
                vm.chunk(Chunk::CLOCK, i.start, i.line as usize); //LOOPl
            }
            GREATER => {
                //SMALLER//SMALLER
                vm.chunk(Chunk::GREATER, i.start, i.line as usize);
            }
            SMALLER => {
                vm.chunk(Chunk::SMALLER, i.start, i.line as usize);
            }
            GREATER_EQ => {
                vm.chunk(Chunk::GREATER_EQ, i.start, i.line as usize);
            }
            ADD_L=>{
                vm.chunk(Chunk::ADD_ARR,i.start,i.line as usize);
            }
            ADD_R=>{
                
            }
            //VARIABLE=>{
                //println!("VAR");
            //}
            R_ARRAY => {
                vm.chunk(Chunk::ARRAY_BEGIN, i.start, i.line as usize);
            }
            L_ARRAY => {
                vm.chunk(Chunk::ARRAY_END, i.start, i.line as usize); // v.next();
            }
            SMALLER_EQ => {
                vm.chunk(Chunk::SMALLER_EQ, i.start, i.line as usize);
            }
            TOKENS_IF_BEFORE => {
                if_index.push(vm.instructions.len()); //l//0
                vm.chunk(Chunk::IGNORE, i.start, i.line as usize);
            }
            CONST => {
                //(d)
                let m = i.snippet.clone(); //*//Some//src
                if m == "oui" {
                    vm.chunk(
                        //value
                        Chunk::CONST(ValueContainer::new_bool(true, 0)),
                        i.start,
                        i.line as usize,
                    ); // index
                } else if m == "non" {
                    vm.chunk(
                        //value
                        Chunk::CONST(ValueContainer::new_bool(false, 0)),
                        i.start,
                        i.line as usize,
                    ); // index
                } else if let Ok(d) = i32::from_str_radix(m.as_str(), 10) {
                    //radix
                    vm.chunk(
                        Chunk::CONST(ValueContainer::new_num(d, 0)),
                        i.start,
                        i.line as usize, //chunk//value//index
                    );
                } else if let Ok(d) = m.parse::<f32>() {
                    //f32::st
                    vm.chunk(
                        //num
                        Chunk::CONST(ValueContainer::new_float(d, 0)), //chunk//value//index
                        i.start,
                        i.line as usize,
                    );
                }
            }
            CONST_STR => {
                let m = escape_chars(&mut i.snippet.clone()); //self.instructions.get(self.instruction)
                vm.chunk(
                    Chunk::CONST(ValueContainer::new_string(m, 0)),
                    i.start,
                    i.line as usize,
                ); //New_f//value, index
            }
            POP=>{//chunk
                vm.chunk(Chunk::POP, i.start,i.line as usize);//start, line
            }
            DIV => {
                vm.chunk(Chunk::DIV, i.start, i.line as usize);
            }
            MOD => {
                vm.chunk(Chunk::MOD, i.start, i.line as usize);
            }
            SUB => {
                vm.chunk(Chunk::SUB, i.start, i.line as usize);
            }
            EQUAL => {
                vm.chunk(Chunk::EQUAL, i.start, i.line as usize);
            }
            ADD => {
                vm.chunk(Chunk::ADD, i.start, i.line as usize);
            }
            MUL => {
                vm.chunk(Chunk::MUL, i.start, i.line as usize); //chunk
            }
            PRINT => {
                vm.chunk(Chunk::PRINT, i.start, i.line as usize);
            }
            POP=>{//Tokens::
                //depth
            }
            IF | LOOP => {
                let is_a_loop = i.token == LOOP; //_if
                let mut k = 0 + 1; //()
                let mut depth = 1 - 1 + 1;
                let mut tok = tokens.clone().into_iter();
                let mut has_else = false;
                tok.nth(n + k - 1); //
                let _d = tok.next();
                //    if let Some(k)=d{
                //
                //println!("pwp {:?}",k.token);
                //         if k.token!=Tokens::BEGIN{
                //              error_compiler(k.start,k.line.try_into().unwrap(),"Un 'alors' est attendu après un 'si'".to_owned());
                //           }
                //        }
                'd: while depth > 0 {
                    if let Some(d) = tok.next() {
                        //n
                        match d.token {
                            
                            Tokens::END => {
                                //  println!("END");
                                depth -= 1;
                            }
                            Tokens::ELSE if !is_a_loop => {
                                println!("*else*");
                                if depth == 1 {
                                    depth = 0;
                                    has_else = true
                                }
                            }
                            Tokens::LOOP => {
                                depth += 1;
                            }
                            Tokens::IF => {
                                // println!("WE NEED TO GO DEEPER!");
                                depth += 1;
                            }
                            Tokens::BEGIN => {
                                //println!("BEG");
                            }
                            _t => {
                                // println!("{:?}",_t);
                            }
                        }
                    } else {
                        break 'd;
                    }
                    k += 1;
                }
                if depth > 0 {
                    fatal!(
                        "EOF obtenu en scannant pour un \"si\" (profondeur {}). Le si, le tant que ou autre se situe à la ligne {}",
                        depth,i.line+1
                    );
                } //0//32//JMPIFFALSE(usize::MAX)
                vm.chunk(Chunk::IGNORE, i.start, i.line as usize); //self.instructions[index]
                                                                   //if is_a_loop{}
                                                                   //println!("R!");//-1
                vm.chunk(Chunk::IGNORE, i.start, i.line as usize);
                backward_jmps.push(JmpBackwardRef {
                   // index: n as usize,
                    index_to: n as usize + k, // + 1 - 1
                    is_loop: is_a_loop,
                    usize_from: vm.instructions.len() - 2, //
                    push_bool: has_else,
                }); //()
                    //for i in
                    //skip=true;
            }
            END => {
                // println!("ENDME");
                //vm.chunk(Chunk::END,i.start,i.line as usize);
            }
            ELSE => {
                let mut k = 1;
                let mut depth = 1;
                let mut tok = tokens.clone().into_iter();
                tok.nth(n + k - 1);
                let _d = tok.next();
                'd: while depth > 0 {
                    if let Some(d) = tok.next() {
                        //n
                        match d.token {
                            Tokens::END => {
                                //  println!("END");
                                depth -= 1;
                            }
                            Tokens::LOOP => {
                                depth += 1;
                            }
                            Tokens::IF => {
                                // println!("WE NEED TO GO DEEPER!");
                                depth += 1;
                            }
                            Tokens::BEGIN => {
                                //println!("BEG");
                            }
                            _t => {
                                // println!("{:?}",_t);
                            }
                        }
                    } else {
                        break 'd;
                    }
                    k += 1;
                }
                if depth > 0 {
                    fatal!(
                        "EOF obtenu en scannant pour un \"else\" (profondeur {})",
                        depth
                    );
                } //0//32//JMPIFFALSE(usize::MAX)
                vm.chunk(
                    Chunk::CONST(ValueContainer {
                        value: crate::vm::Value::BOOL(true),
                        index: 0,
                    }),
                    i.start,
                    i.line as usize,
                );
                vm.chunk(Chunk::IGNORE, i.start, i.line as usize); //self.instructions[index]
                                                                   //if is_a_loop{}
                                                                   //println!("R!");//-1
                backward_jmps.push(JmpBackwardRef {
                   // index: n as usize,
                    index_to: n as usize + k, // + 1 - 1
                    is_loop: false,
                    usize_from: vm.instructions.len() - 1, //
                    push_bool: false,
                }); //()
                    //for i in
                    //skip=true;
            }
            VARIABLE => {
                if !hashmap_vars.contains_key(&i.snippet) {
                    //k//&
                    hashmap_vars.insert(i.snippet.clone(), var_count + 1); //+//+=1//s
                    var_count += 1;
                } //i.snipet.clone()
                vm.chunk(
                    //*hashmap_vars.get(&i.snippet).unwrap()
                    Chunk::VARIABLEREF(i.snippet.clone()),
                    i.start,
                    i.line as usize,
                ); //_
            }
            VARIABLE_ASSIGN => {
                if !hashmap_vars.contains_key(&i.snippet) {
                    //k//&
                    hashmap_vars.insert(i.snippet.clone(), var_count + 1); //+//+=1//s
                    var_count += 1;
                } //i.snipet.clone()///.clone()//_
                vm.chunk(
                    Chunk::VARIABLEASS(i.snippet.clone()), //*hashmap_vars.get(&i.snippet).unwrap()
                    i.start,
                    i.line as usize,
                );
            }
            ASSIGN => {
                vm.chunk(Chunk::ASSIGN, i.start, i.line as usize);
            }
            //VARIABLE_ASSIGN=>{
            //    vm.chunk(Chunk::VARIABLEASSIGN(i.snipet.clone()),i.start,i.line as usize);
            // }
            EOF => {
                break;
            }
            _ => {
                println!("Attention: Ce ({:?}) token n'est pas reconnu", i.token);
                //Erreur
            }
        }
        n += 1;
    }
}
//parser
pub fn error_compiler(char_index: usize, line_number: usize, message: String) {
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
                for _i in 0..(r.0 - ofs - 1 - 1) {
                    //
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
pub const ERROR_INSTANTLY_QUITS: bool = true;

pub fn escape_chars(d: &mut String) -> String {
    let mut c = d.clone();

    c = c.replace("\\n", "\n"); //from// to
    c = c.replace("\\t", "\t");
    c //d
}
