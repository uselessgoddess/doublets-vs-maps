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
            table.remove(&i).unwrap();
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
            table.remove(&i).unwrap();
        }
    })
}

#[bench]
fn doublets(b: &mut Bencher) -> impl Termination {
    let mut table = doublets::mem::splited::Store::<u64, _, _>::with_constants(
        GlobalMem::new().unwrap(),
        GlobalMem::new().unwrap(),
        LinksConstants::external(),
    )
    .unwrap();
    let any = table.constants().any;
    b.iter(|| {
        for i in 1..=1_000_000_u64 {
            table.create_link(i, i).unwrap();
        }
        for i in 1..=1_000_000_u64 {
            table.delete(i).unwrap();
        }
    })
}
