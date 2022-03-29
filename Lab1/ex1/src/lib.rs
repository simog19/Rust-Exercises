pub fn capitalize(s: &str) -> String {
    let mut s0 = String::new();
    let space = String::from(" ");

    let v: Vec<&str> = s.split(' ').collect();

    for st in v {
        let mut c = st.chars(); 
        let z = match c.next(){
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        s0.push_str(&z);
        s0.push_str(&space);
    }

    let s2 = s0.trim();
    return s2.to_string();
}
