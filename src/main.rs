use hash_table_rs::benchmarks::*;
use std::time::Instant;
fn main() {
    const N: usize = 100_000;

    let begin = Instant::now();
    benchmark_the_crate_table(N);
    println!("Crate HashTable :: {:>8.5}s", begin.elapsed().as_secs_f32());

    let begin = Instant::now();
    benchmark_std_table(N);
    println!("'std' HashTable :: {:>8.5}s", begin.elapsed().as_secs_f32());
}
