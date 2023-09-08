use std::{env, fs::File};

use compiler::compile_to_bitcode;
use lexer::lex;
use parser::parse_rearrange;
use vm::MachineVirtuelle;
//_to_instructions
use std::fs::{read_to_string, read};
use std::io::Write;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod vm;
//use lazy_static::lazy_static;
//lazy_static!{
//pub static ref _indepth_debug:bool=false;
//}
pub fn load_executable(path:&str){
    let rts = read(path.clone().to_string()+".vbe");//_to
    if let Ok(d) = rts {
        let mut d=MachineVirtuelle::deserialize(d);//P
        d.start();
    } else if let Err(d) = rts {//P
        println!("Impossible de charger le programme {}\n {}", path.clone().to_string()+".vbe", d);//fichier
    }
}
pub fn load_file(path: &str, compile_only: bool, out_path: String) {
    //&str
    let rts = read_to_string(path);
    if let Ok(d) = rts {
        //=//Some
        let m = lex(d.clone());
        let k = parse_rearrange(&m); //lexer//to_instructions
        let mut n = MachineVirtuelle::new(Some(d));
        compile_to_bitcode(&k, &mut n); //tokens//vm
        if !compile_only {
            println!("\n - - - \n"); //m
            n.start();
        } else {
            // let path = "results.txt";
            let output = File::create(out_path.clone()+".vbe"); //path//?//mut
            if let Ok(mut output) = output {
                //o
                let line = n.serialize(); //"hello";
                let c=write!(output, "{}", line);
                if let Err(err)=c{
                    println!("Erreur lors de l'écriture du fichier \n{}",err);
                }
                else{
                    println!("Fichier {} compilé vers {} avec succès!",path,out_path+".vbe");
                }

            } else if let Err(err) = output {
                //
                println!("Erreur lors de l'écriture du fichier \n{}", err);
            }
        }
    } else if let Err(d) = rts {
        println!("Impossible de charger le fichier {}\n {}", path, d);
    }
}
fn main() {
    //for arg in env::args(){
    //    println!("{}",arg);
    //}
    let mut args = env::args();
    args.next();
    //println!("{:?}",args);
    let arglen = args.len();
    //if args.len() == 0 {
        //println!("Mode REPL...");
    //} else 
    if args.len() > 0 {
        //(args.len() == 1)
        let mut str_load = String::from("");
        let mut compile = false;
        let mut flag = false;
        let mut out_path = String::new();
        let mut specify = false;
        let mut execute=false;
        for i in args {
            //println!("WAHOO");
            if i == String::from("-c") {
                compile = true;
                continue;
            } else if i == String::from("-d") {
                flag = true;
            } else if i == String::from("-s") {
                specify = true;
            } else if i==String::from("-e"){
                execute=true;
            } 
            else if specify {
                out_path = i;
                specify = false;
            } else if flag && i.contains("=") {
                //compile//pat
                let f = i.clone(); //mut
                let mut m = f.split(&"=").into_iter(); //pat
                let mut c = 0; //f.split("=");
                for _i in m.clone() {
                    //&
                    c += 1;
                }
                if c == 2 {
                    let a = m.next().unwrap();
                    let b = m.next().unwrap();
                    match a {
                        "erreur" => {
                            match b.to_lowercase() {
                                _ => {
                                    println!("Valuer inconnue pour le paramètre 'erreur'")
                                }
                                //String::from("oui")=>{
                                //
                                //}//
                            }
                        }
                        _ => {
                            println!("Paramètre inconnu: {} (valeur {})", a, b); //i//obtenu//a
                            return;
                        }
                    }
                    //for i in m{
                    //
                    //                   }
                } else if c > 2 {
                    println!("Les paramètres de drapeaux s'écrivent ainsi: 'drapeau=valeur'. Le paramètre obtenu est {}",i);
                    return;
                } else {
                    str_load = i.clone();
                    //continue;
                }
            } else {
                str_load = i.clone();
                //continue;
            }
        }
        if execute&&!str_load.is_empty(){
            load_executable(str_load.as_str());//and_ex
            return;
        }
        else if compile && !str_load.is_empty() {
            //i
            if out_path.is_empty() {
                println!("Aucun chemin de sortie n'as été spécifié"); //fatal
            } else {
                load_file(str_load.as_str(), compile, out_path); //path
                return; //
            }
        } else if !str_load.is_empty() {
            load_file(str_load.as_str(), compile, out_path);
            return;
        } else {
        }
    } // else {////C//c//compiler seulement//\n 
    println!("Usage: verbose (fichier)\t <-- exécuter un fichier\n       verbose\t\t\t <-- REPL\nParamètres: \n       -c\t\t\t\t<-- compiler seulement\n       -d\t\t\t\t<-- activer des paramètres et drapeaux\n       -s\t\t\t\t<-- Spécifier la sortie\n       -e\t\t\t\t<-- Exécuter un programme\nParamètres reçus: {}",arglen);
    //s.len()
    //  }
    //   println!("Hello, world!");
}

#[macro_export]
macro_rules! fatal {//vec
   // ( $x:expr,$($arg:tt)* ) => {
    //    {
    //      println!($x,$arg.collect());//;
    //      std::process::exit(0)//sys::exi
    //    }
    //};
    () => ({print!("\n");std::process::exit(-1)});
    ($fmt:expr) => ({print!(concat!($fmt, "\n")); std::process::exit(-1)});
    ($fmt:expr, $($arg:tt)*) =>({ print!(concat!($fmt, "\n"), $($arg)*); std::process::exit(-1) });//flegme
}
