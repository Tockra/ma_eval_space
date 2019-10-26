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

    if args.len() != 6 {
        println!("Bitte verwende {} <stree|rbtree|btree|binary> <u40|u48|u64> <uniform|normal|normal|bwt_runs> <variant=0,..,32> <name>",args[0]);
        return;
    }
	
    if args[3] != "uniform" && args[3] != "normal" && args[3] != "bwt_runs"   {
        println!("Bitte verwende {} <stree|rbtree|btree|binary> <u40|u48|u64> <uniform|normal|bwt_runs> <variant=0,..,32> <name>",args[0]);
        return;
    } 

	match args[2].as_ref() {
		"u40" => stage1::<u40>(args),
		"u48" => stage1::<u48>(args),
		"u64" => stage1::<u64>(args),
		_ => println!("Bitte verwende {} <stree|rbtree|btree|binary> <u40|u48|u64> <uniform|normal|bwt_runs> <variant=0,..,32> <name>",args[0]),
    }
}

fn stage1<T: 'static + Int + Typable + Default + num::Bounded + From<u64> + Copy + Debug>(args: Vec<String>) {
    match args[1].as_ref() {
        "stree" => stage2::<T,STree<T>>(args[3].as_ref(), args[4].as_ref(), args[5].as_ref()),
        "rbtree" => stage2::<T,RBTree<T>>(args[3].as_ref(), args[4].as_ref(), args[5].as_ref()),
        "btree" => stage2::<T,BTreeMap<T,T>>(args[3].as_ref(), args[4].as_ref(), args[5].as_ref()),
		"binary" => stage2::<T,BinarySearch<T>>(args[3].as_ref(), args[4].as_ref(), args[5].as_ref()),
        _ => println!("Bitte verwende {} <stree|rbtree|btree|binary> <u40|u48|u64> <uniform|normal|bwt_runs> <variant=0,..,32> <name>",args[0]),
    }
}

fn stage2<T: 'static + Int + Typable + From<u64> + Copy + Debug, U: PredecessorSetStatic<T>>(arg: &str, var: &str, name: &str) {
    measure::<T,U>(arg, var.parse::<u32>().unwrap(), name);
}