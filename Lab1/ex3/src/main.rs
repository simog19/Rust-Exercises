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


    /*

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

const FAIL: &str = "bad board format";

fn index(x: usize, y: usize, dim_x: usize) -> usize {
    x*dim_x + y
}

fn count(board: &Vec<u8>, x: usize, y: usize, dim_x: usize, dim_y: usize) -> u8 {
    let mut count = 0;
    for i in if x>0 {x-1} else {0}..if x < dim_x -1 {x+1} else {dim_x} {
        for j in if y>0 {y-1} else {0}..if y < dim_y -1 {y+1} else {dim_y} {
            count += if board[index(i, j, dim_x)] == '*' as u8 {1} else {0};
        }
    }
    count
}


fn main() {
    let args = Args::parse();

    if args.board.len() != args.rows as usize * args.cols as usize {
        panic!("{}", FAIL);
    }

    let mut bytes: Vec<u8> = args.board.into_bytes();
    for i in 0..args.cols {
        for j in 0..args.rows {
            let off = index(i, j, args.cols);
            if bytes[off] == ' ' as u8 {
                bytes[off] = match count(&bytes, i, j, args.cols, args.rows) {
                    0  => '0' as u8,
                    x @ 1..=6  => '0' as u8 + x,
                    _ => panic!("bad count")
                };
            }
        }
    }

    println!("{}", String::from_utf8(bytes).unwrap());

}
    */

}