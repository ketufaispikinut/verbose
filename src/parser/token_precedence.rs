//use crate::fatal;

pub fn infix_binding_power(t: char) -> Option<(u8, u8)> {
    let d = match t {
        '=' => (2, 1),
        '<' | '>'=>(3, 4),//<
        '_' => (4, 5), //=//2//1
        '+' => (6, 7),
        '-' => (8, 9), //| '-'

        '*' | '/' | '%' => (9, 10),
        //  =>{

        //  }
        _ => {
            return None;
            //  fatal!("o non: {}",t)//;
        }
    };
    Some(d)
}
pub fn prefix_binding_power(t: char) -> Option<((), u8)> {
    let d = match t {
        'p' => {
            //2
            ((), 0) //0//1
        }
        'i' => ((), 0),
        'w' => ((), 0),
        _ => {
            return None;
        }
    }; //p
    Some(d) //;
            //None
}
