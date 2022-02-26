use hash_table_rs::benchmarks::*;
use std::time::Instant;
fn main() {
    const N: usize = 100_000;
    let mut i = 10;
    let begin = Instant::now();
    loop {
        if i == 0 {
            break;
        }
        benchmark_the_crate_table(N);
        i -= 1;
    }
    println!("Crate HashTable :: {:>8.5}s", begin.elapsed().as_secs_f32());

    let begin = Instant::now();
    i = 10;
    loop {
        if i == 0 {
            break;
        }
        benchmark_std_table(N);
        i -= 1;
    }
    println!("'std' HashTable :: {:>8.5}s", begin.elapsed().as_secs_f32());
}
