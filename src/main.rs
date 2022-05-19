#![feature(bench_black_box)]

use doublets::mem::splited::Store;
use doublets::Doublets;
use platform_mem::GlobalMem;
use rand::Rng;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryInto;
use std::hint::black_box;
use std::time::Instant;

trait Map {
    fn len(&mut self) -> usize;

    fn insert(&mut self, key: u64, value: u64) -> Option<u64>;

    fn get(&mut self, key: &u64) -> Option<u64>;
}

impl Map for BTreeMap<u64, u64> {
    fn len(&mut self) -> usize {
        BTreeMap::len(self)
    }

    fn insert(&mut self, key: u64, value: u64) -> Option<u64> {
        BTreeMap::insert(self, key, value)
    }

    fn get(&mut self, key: &u64) -> Option<u64> {
        BTreeMap::get(self, key).map(|v| v.clone())
    }
}

impl Map for HashMap<u64, u64> {
    fn len(&mut self) -> usize {
        HashMap::len(self)
    }

    fn insert(&mut self, key: u64, value: u64) -> Option<u64> {
        HashMap::insert(self, key, value)
    }

    fn get(&mut self, key: &u64) -> Option<u64> {
        HashMap::get(self, key).map(|v| v.clone())
    }
}

impl Map for Store<u64, GlobalMem, GlobalMem> {
    fn len(&mut self) -> usize {
        self.count().try_into().unwrap()
    }

    fn insert(&mut self, key: u64, value: u64) -> Option<u64> {
        if self.search(key, value).is_none() {
            self.create_link(key, value).ok()
        } else {
            None
        }
    }

    fn get(&mut self, key: &u64) -> Option<u64> {
        self.search(key.clone(), self.constants().any)
    }
}

fn bench<M: Map, R: Rng>(m: &mut M, max: u64, repeats: u64, count: u64, rng: &mut R) {
    let uniform = 2..max;
    let mut sum: u8 = 0;

    for k in 0..count {
        let instant = Instant::now();
        for _ in 0..repeats {
            let k = rng.gen_range(uniform.clone());
            m.insert(k, k - 1);
        }
        println!(
            "count: {} len: {} time: {:?}",
            (k + 1) * repeats,
            m.len(),
            instant.elapsed()
        );
    }

    for k in 0..count {
        let mut misses = 0;
        let instant = Instant::now();
        for _ in 0..repeats {
            let k = rng.gen_range(uniform.clone());
            match m.get(&k) {
                None => {
                    misses += 1;
                }
                Some(v) => {
                    sum = sum.wrapping_add(v as u8);
                }
            }
        }
        println!(
            "iterations: {} misses: {} time: {:?}",
            (k + 1) * repeats,
            misses,
            instant.elapsed()
        );
    }

    for _ in 0..count {
        let instant = Instant::now();
        for _ in 0..repeats {
            black_box(rng.gen_range(uniform.clone()));
        }
        // println!("rand time: {:?}", instant.elapsed());
    }
}

fn main() {
    let max = 100_000;
    let repeats = 1_000_000;
    let count = 10;

    let mut rng = rand::thread_rng();
    println!("BTreeMap");
    {
        let mut m = BTreeMap::new();
        bench(&mut m, max, repeats, count, &mut rng);
    }
    println!("HashMap");
    {
        let mut m = HashMap::new();
        bench(&mut m, max, repeats, count, &mut rng);
    }
    println!("Doublets");
    {
        let mut m =
            Store::<_, _, _>::new(GlobalMem::new().unwrap(), GlobalMem::new().unwrap()).unwrap();
        bench(&mut m, max, repeats, count, &mut rng);
    }
}
