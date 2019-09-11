extern crate stats_alloc;
extern crate serde;
extern crate rmp_serde as rmps;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use std::alloc::System;
use std::fmt::Debug;
use std::fs::{OpenOptions, File, read_dir};
use std::io::{BufWriter, BufReader, Write};
use ma_titan::default::immutable::STree;
use ma_titan::internal::PredecessorSetStatic;

use uint::u40;
use uint::Typable;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use rmps::Deserializer;
#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    
    measure::<u40,STree<u40>>();
    // Used here to ensure that the value is not
    // dropped before we check the statistics
    //::std::mem::size_of_val(&x);
}

fn measure<E: 'static + Typable + Copy + Debug + From<u64> + DeserializeOwned, T: PredecessorSetStatic<E>>() {
    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("stats.txt").unwrap());

    for dir in read_dir(format!("../ma_titan/testdata/{}/", E::TYPE)).unwrap() {
        let dir = dir.unwrap();
        let path = dir.path();

        let buf = BufReader::new(File::open(path).unwrap());
        
        
        let mut values = Deserializer::new(buf);
        let values: Vec<E> = Deserialize::deserialize(&mut values).unwrap();
        let len = values.len();

        let reg = Region::new(&GLOBAL);
        let _ = T::new(values);
        let change = reg.change();

        let build_size = change.bytes_allocated + change.bytes_allocated.max(0);
        let final_size = change.bytes_allocated as isize + change.bytes_reallocated + change.bytes_deallocated as isize;
        writeln!(result, "RESULT algo={} method=new size={} build_size_bytes={} size_bytes={}",T::TYPE,len,build_size,final_size ).unwrap(); 
    }
}
