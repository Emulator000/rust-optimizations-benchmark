use std::time::Instant;

mod base64;
mod brainfuck;
mod json;
mod matmul;
mod primes;

fn main() {
    let now = Instant::now();
    let _ = brainfuck::execute();
    println!("[brainfuck] elapsed: {:.2?}", now.elapsed());
    println!();

    let now = Instant::now();
    let _ = base64::execute();
    println!("[base64] elapsed: {:.2?}", now.elapsed());
    println!();

    let now = Instant::now();
    let _ = matmul::execute();
    println!("[matmul] elapsed: {:.2?}", now.elapsed());
    println!();

    let now = Instant::now();
    let _ = json::execute();
    println!("[json] elapsed: {:.2?}", now.elapsed());
    println!();

    let now = Instant::now();
    let _ = primes::execute();
    println!("[primes] elapsed: {:.2?}", now.elapsed());
}
