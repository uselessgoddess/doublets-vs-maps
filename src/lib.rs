#![feature(termination_trait_lib)]
#![feature(test)]

extern crate test;

use doublets::Doublets;
use platform_data::LinksConstants;
use platform_mem::GlobalMem;
use std::collections::{BTreeMap, HashMap};
use std::process::Termination;
use test::Bencher;

#[bench]
fn hash(b: &mut Bencher) -> impl Termination {
    let mut table = HashMap::new();
    b.iter(|| {
        for i in 1..=1_000_000_u64 {
            table.insert(i, i);
        }
        for i in 1..=1_000_000_u64 {
            table.remove(&i);
        }
    })
}

#[bench]
fn rb(b: &mut Bencher) -> impl Termination {
    let mut table = BTreeMap::new();
    b.iter(|| {
        for i in 1..=1_000_000_u64 {
            table.insert(i, i);
        }
        for i in 1..=1_000_000_u64 {
            table.remove(&i);
        }
    })
}

#[bench]
fn doublets(b: &mut Bencher) -> impl Termination {
    let mut table = doublets::mem::united::Store::<u64, _>::with_constants(
        GlobalMem::new().unwrap(),
        LinksConstants::external(),
    )
    .unwrap();
    let any = table.constants().any;
    b.iter(|| {
        let mut keys = Vec::with_capacity(1_000_000);
        let mut latest = None;
        for _ in 0..1_000_000 {
            let key = table.create_point().unwrap();
            if let Some(latest) = latest {
                table.create_link(key, latest).unwrap();
            }
            latest = Some(key);
            keys.push(key);
        }
        for key in keys {
            table.delete(key).unwrap();
            if let Some(val) = table.search(key, any) {
                table.delete(val).unwrap();
            }
        }
    })
}
