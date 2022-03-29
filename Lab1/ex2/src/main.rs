use clap::Parser;
//use ex1::capitalize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    sequence: String,
}

fn main() {
    let args = Args::parse();
    let s: String = args.sequence;

    //println!("{}",capitalize(&s)); // oppure println!("{}", capitalize(s.as_str))
}