use std::process::exit;
use clap::Parser;
use luhn::is_valid;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    sequence: String,
}

fn main() {
    let args = Args::parse();
    let s: String = args.sequence;
    let v: Vec<&str> = s.split(' ').collect(); //splitting sequence passed by CLI into groups

    //check if there are more than 4 groups of numbers
    if v.len() != 4 {
        println!("Invalid format!");
        exit(-1);
    }
    else {
        //iterating on each group
        for st in v {
            /*let mut c = st.chars();*/
            //check if the group size is different from 4
            if st.chars().count() != 4 {
                println!("Invalid format!");
                exit(-1);
            } else { //check if an alphabetic symbol is present in the group
                for x in st.chars(){
                    if x.is_alphabetic(){
                        println!("Invalid format!");
                        exit(-1);
                    }
                }
            }
        }
    }

    //calling validation function
    if is_valid(&s) {// either is_valid(s.as_str))
        println!("Valid!");
    }
    else { println!("Not valid!"); }

}