/// Check a Luhn checksum. -- Only for credit card number
/*
pub fn is_valid(code: &str) -> bool {
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
*/
pub fn is_valid(code: &str) -> bool {

    let mut digits = String::new();
    for c in code.chars() {
        match c {
            c if c.is_digit(10) => digits.push(c),
            ' ' => (),
            _ => {
                return false;
            }
        }
    }

    if digits.len() < 2 {
        return false;
    }

    let mut sum = 0;
    for (idx, sdigit) in digits.chars().rev().enumerate() {
        let num = sdigit.to_digit(10).unwrap();
        sum += if idx % 2 == 0 {
            num
        } else if num * 2 > 9 {
            num * 2 - 9
        } else {
            num * 2
        };
    }

    sum % 10 == 0
}

