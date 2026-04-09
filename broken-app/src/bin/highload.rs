use broken_app::{algo, normalize};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Function to benchmark: normalize, fib, dedup
    func: String,
}

fn gen_big_string(seed: u64, len: usize) -> String {
    let chars = b"abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ\t\n  ";
    let mut s = String::with_capacity(len);
    let mut state = seed;
    for _ in 0..len {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        s.push(chars[(state >> 33) as usize % chars.len()] as char);
    }
    s
}

fn gen_big_slice(seed: u64, len: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(len);
    let mut state = seed;
    for _ in 0..len {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        v.push((state >> 33) % 1000);
    }
    v
}

fn main() {
    let args = Args::parse();

    match args.func.as_str() {
        "normalize" => {
            let text = gen_big_string(42, 500_000);
            println!("normalize len: {}", normalize(&text).len());
        }
        "fib" => {
            let n = 45;
            let fib = algo::slow_fib(n);
            println!("fib({n}): {fib}");
        }
        "dedup" => {
            let data = gen_big_slice(42, 10_000);
            let uniq = algo::slow_dedup(&data);
            println!("dedup len: {}", uniq.len());
        }
        other => eprintln!("Unknown function: {other}. Use: normalize, fib, dedup"),
    }
}
