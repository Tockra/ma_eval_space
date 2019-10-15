mod bench;
use bench::*;
use ma_titan::default::immutable::{Int, STree};
use uint::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
	let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        println!("Bitte genau drei Argumente übergeben!");
    }
	
	match args[2].as_ref() {
		"u40" => stage1::<u40>(args),
		"u48" => stage1::<u48>(args),
		"u64" => stage1::<u64>(args),
		_ => panic!("Bitte verwende {} <stree|vebtree|btree|binary> <u40|u48|u64> <uniform|normal/bereich_viertel|normal/bereich_komplett|bwt_runs>",args[0]),
    }
}

fn stage1<T: Int + Typable + From<u64> + Copy + Debug>(args: Vec<String>) {
    match args[1].as_ref() {
        "stree" => stage2::<T,STree<T>>(args[3].as_ref()),
        "vebtree" => stage2::<T,VEBTree>(args[3].as_ref()),
        "btree" => stage2::<T,BTreeMap<T,T>>(args[3].as_ref()),
		"binary" => stage2::<T,BinarySearch<T>>(args[3].as_ref()),
        _ => panic!("Bitte verwende {} <stree|vebtree|btree|binary> <u40|u48|u64> <uniform|normal/bereich_viertel|normal/bereich_komplett|bwt_runs>",args[0]),
    }
}

fn stage2<T: Int + Typable + From<u64> + Copy + Debug, U: PredecessorSetStatic<T>>(arg: &str) {
    measure::<T,U>(arg);
}