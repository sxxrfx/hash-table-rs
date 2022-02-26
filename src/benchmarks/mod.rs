use crate::hash_table::*;

pub fn benchmark_the_crate_table(n: usize) {
    // use crate::hashTables::*;
    let mut hash = HashTable::<usize, usize>::new();
    for _ in 0..n {
        let key = rand::random::<usize>();
        if let Some(value) = hash.get_mut(&key) {
            *value += 1;
        } else {
            hash.insert(key, 1);
        }
    }
}

pub fn benchmark_std_table(n: usize) {
    use std::collections::HashMap;

    let mut hash = HashMap::<usize, usize>::new();
    for _ in 0..n {
        let key = rand::random::<usize>();
        if let Some(value) = hash.get_mut(&key) {
            *value += 1;
        } else {
            hash.insert(key, 1);
        }
    }
}
