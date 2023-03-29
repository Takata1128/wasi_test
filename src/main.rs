use clap::Parser;
use wasi_test::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    benchmarks: String,
}

fn main() {
    let args = Args::parse();
    if args.benchmarks == "fib" {
        println!("{}", fib(100000000));
    } else if args.benchmarks == "fib_rec" {
        println!("{}", fib_rec(45));
    } else if args.benchmarks == "hello" {
        hello();
    }
}
