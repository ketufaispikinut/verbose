//use crate::fatal;

pub fn infix_binding_power(t: char) -> Option<(u8, u8)> {
    let d = match t {
        '=' => (2, 1),
        '<'|'>'=>(3,4),//<
        '_' => (3+1, 4+1), //=//2//1
        '+' => (5+1, 6+1),
        '-' => (7+1, 8+1), //| '-'

        '*' | '/' | '%' => (7 + 1+1, 8 + 1+1),
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
