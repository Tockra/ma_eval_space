extern crate stats_alloc;
extern crate serde;
extern crate rmp_serde as rmps;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use std::alloc::System;
use std::fmt::Debug;
use std::fs::{OpenOptions, File, read_dir};
use std::io::{BufWriter, BufReader, Write};
use ma_titan::default::immutable::STree;
use ma_titan::benches::BinarySearch;

use ma_titan::internal::PredecessorSetStatic;
use std::time::{Instant};

use uint::u40;
use uint::Typable;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use rmps::Deserializer;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    let now = Instant::now();
    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("stats.txt").unwrap());

    // Messen für STree<u40>
    measure::<u40,STree<u40>>(&mut result);

    // Messen für BinarySearch<u40> (Baseline)
    measure::<u40,BinarySearch<u40>>(&mut result);

    
    // Used here to ensure that the value is not
    // dropped before we check the statistics

    println!("Ausführungsdauer {}", now.elapsed().as_secs());
    
}

/// Diese Methode dient der Hauptspeichermessung der new()-Methode verschiedener zu untersuchender Datenstrukturen E
/// mit elementen E = {u40,u48,u64} .
fn measure<E: 'static + Typable + Copy + Debug + From<u64> + DeserializeOwned, T: PredecessorSetStatic<E>>(result: &mut BufWriter<File>) {

    for dir in read_dir(format!("../ma_titan/testdata/{}/", E::TYPE)).unwrap() {
        let dir = dir.unwrap();
        let path = dir.path();
        println!("{:?}",path);

        let buf = BufReader::new(File::open(path).unwrap());
        
        
        let mut values = Deserializer::new(buf);

        let mut reg = Region::new(&GLOBAL);
        let values: Vec<E> = Deserialize::deserialize(&mut values).unwrap();
        let len = values.len();

 
        let x = T::new(values);
        let change = reg.change_and_reset();

        // Das Ergebnis wird in die stats.txt geschrieben, die von SQLPlots analysiert und geplottet werden kann
        writeln!(result, "RESULT data_structure={} method=new size={} build_size_bytes={} size_bytes={}",T::TYPE,len,change.bytes_max_used,change.bytes_current_used + std::mem::size_of_val(&x) ).unwrap(); 
    }
}