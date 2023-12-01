use std::{fs,env,str};



fn main() {
    let mut total = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    let codes:Vec<&str>= input.split('\n').collect();
    for code in codes{
        let mut chars = code.as_bytes();
        let mut lp =  0;
        let mut rp = code.len() - 1;
        let mut left_val:Option<char> = None;
        let mut right_val:Option<char> = None;

        while (lp < code.len()){
            let left = chars[lp] as char;
            let right = chars[rp] as char;
            if left.is_numeric() & (left_val == None){
                left_val = Some(left);
            }
            

            if right.is_numeric() & (right_val == None){
                right_val = Some(right);
            }
            lp = lp + 1;
            if rp > 0{
                rp = rp - 1;
            }
            
        }
        let mut val = 0;
        match (left_val,right_val) {
            (Some(l),Some(r))=>{
                 val = l.to_digit(10).unwrap() * 10 + r.to_digit(10).unwrap();
            }
            _=> {}
        }
        println!("{}",val);
        total = total + val;
    }

    println!("Content:\n{total}")
}
