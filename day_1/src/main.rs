use std::{env, fs, str,collections::HashMap, string};



fn main() {

    let mut total = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    let codes: Vec<&str> = input.split('\n').collect();

    let map: HashMap<&str, char> = HashMap::from([
        ("zero",'0'),
        ("one",'1'),
        ("two",'2'),
        ("three",'3'),
        ("four",'4'),
        ("five",'5'),
        ("six",'6'),
        ("seven",'7'),
        ("eight",'8'),
        ("nine",'9'),
    ]);

    
    for  code in codes {
        
        println!("{}",code);
        let  chars = code.as_bytes();
        let mut lp = 0;
        let mut rp = code.len() - 1;
        let mut left_val: Option<char> = None;
        let mut right_val: Option<char> = None;

        while lp < code.len() {
            let left = chars[lp] as char;
            let right = chars[rp] as char;
            if left.is_numeric() & (left_val == None) {
                left_val = Some(left);
            }

            if right.is_numeric() & (right_val == None) {
                right_val = Some(right);
            }
            if code.len()>=5{
                if (lp < code.len()-5)& (left_val == None){
                    for i in 3..6{
                        if map.contains_key(&code[lp..(lp+i)]){
                            left_val = Some(*map.get(&code[lp..(lp+i)]).unwrap())
                        }
                    }
                }
                if (code.len() >=5)&(rp < code.len()-5 ) & (right_val  == None){
                    for i in 3..6{
                        if map.contains_key(&code[rp..(rp+i)]){
                            right_val  = Some(*map.get(&code[rp..(rp+i)]).unwrap())
                        }
                    }
                }
            }
            if code.len() >= 4{
                if (code.len() >=4)&(lp < code.len()-4)& (left_val == None) {
                    for i in 3..5{
                        if map.contains_key(&code[lp..(lp+i)]){
                            left_val = Some(*map.get(&code[lp..(lp+i)]).unwrap())
                        }
                    }
                }
                if (code.len() >=4)&(rp < code.len()-4 ) & (right_val  == None) {
                    for i in 3..5{
                        if map.contains_key(&code[rp..(rp+i)]){
                            right_val  = Some(*map.get(&code[rp..(rp+i)]).unwrap())
                        }
                    }
                }
            }
            if code.len() >= 3{
                if  (code.len() >=3)&(lp < code.len()-3)& (left_val == None) {
                    for i in 3..4{
                        if map.contains_key(&code[lp..(lp+i)]){
                            left_val = Some(*map.get(&code[lp..(lp+i)]).unwrap())
                        }
                    }
                }
                
                
                if  (code.len() >=3)&(rp < code.len()-3 ) & (right_val  == None){
                    for i in 3..4{
                        if map.contains_key(&code[rp..(rp+i)]){
                            right_val  = Some(*map.get(&code[rp..(rp+i)]).unwrap())
                        }
                    }
                }
            }
            lp = lp + 1;
            if rp > 0 {
                rp = rp - 1;
            }
            
        }
        let mut val = 0;
        match (left_val, right_val) {
            (Some(l), Some(r)) => {
                val = l.to_digit(10).unwrap() * 10 + r.to_digit(10).unwrap();
            }
            _ => {}
        }
        println!("{}", val);
        total = total + val;
    }

    println!("Content:\n{total}")
}
