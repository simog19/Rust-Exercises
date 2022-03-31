/// Check a Luhn checksum. -- Only for credit card number

pub fn is_valid(code: &str) -> bool {
    //checks on the code format are commented because they are performed in the main fn
    //check if there are more than 4 groups of numbers

    /*
    let v: Vec<&str> = code.split(' ').collect(); //splitting sequence passed by CLI into groups

    if v.len() != 4 {
        return false;
    }
    else {
        //iterating on each group
        for st in v {
            /*let mut c = st.chars();*/
            //check if the group size is different from 4
            if st.chars().count() != 4 {
                return false;
            } else { //check if an alphabetic symbol is present in the group
                for x in st.chars(){
                    if x.is_alphabetic(){
                        return false;
                    }
                }
            }
        }
    }*/
    let cc = code.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let mut s = String::new();

    let mut i = 0;
    for d in cc.chars() {
        let mut n = d.to_digit(10).unwrap();
        if i % 2 == 0 {
            n = n*2;
            if n > 9 {
                n = n-9;
            }
            s.push_str(&n.to_string());
        }
        else{ s.push(d) }
        i += 1;
    }
    let sum = s.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();

    if sum % 10 == 0 {
       return true;
    }
    return false;
}

