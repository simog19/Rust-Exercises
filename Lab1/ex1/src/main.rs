/*
use std::env::args; //command line parameter are accessible by means of args, that is a system iterator
// it means that args function return an iterable collection, so i can use collect() method on it.
// args.len() gives to me the number of parameter
// args.next() let me to pass to next one, this method return an optional (it could be a string or nothing, if it's nothing we are at the bottom)
// args.skip() let us to skip the next n parameters (n passed to skip)

*/
use clap::Parser;
use ex1::capitalize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    sequence: String,
}

fn main() {
    let args = Args::parse();
    let s: String = args.sequence;
    /*
        let args: Vec<String> = args().skip(1).collect(); // the first block is the executable name, like always
        // se non sapessi in che tipo di vettore mettere cio che prende collect, usare la notazione seguente
        //args.collect::Vec<String>, dicendo al compilatore che tipo di vettore mi deve fare collect
         */
    println!("{}",capitalize(&s)); // oppure println!("{}", capitalize(s.as_str))
}
