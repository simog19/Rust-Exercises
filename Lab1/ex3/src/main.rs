use minesweeper::annotate;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    board: String,

    #[clap(short, long)]
    rows: usize,

    #[clap(short, long)]
    cols: usize
}


fn main() {
    let minefield = vec![
        " * * ", //&str
        "  *  ", //&str
        "  *  ", //&str
        "     ",
    ];
    println!("{:?}", annotate(&minefield));

}