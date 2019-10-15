extern crate stats_alloc;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use std::fmt::Debug;
use std::fs::{OpenOptions, File, read_dir};
use std::io::{BufWriter, Write, Read};
use std::alloc::System;
use ma_titan::default::immutable::STree;
use ma_titan::benches::BinarySearch;

use ma_titan::internal::PredecessorSetStatic;
use std::time::{Instant};

use uint::u40;
use uint::Typable;

use std::env;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Bitte genau ein Argument übergeben!");
    }

    let now = Instant::now();
    match args[1].as_ref() {
        "normal_komplett" => {
            let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open("stats_normal_komplett.txt").unwrap());

            // Messen für STree<u40>
            measure_normal_komplett::<u40,STree<u40>>(&mut result);

            // Messen für BinarySearch<u40> (Baseline)
            measure_normal_komplett::<u40,BinarySearch<u40>>(&mut result);
        },
        "normal_viertel" => {
            let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open("stats_normal_viertel.txt").unwrap());

            // Messen für STree<u40>
            measure_normal_viertel::<u40,STree<u40>>(&mut result);

            // Messen für BinarySearch<u40> (Baseline)
            measure_normal_viertel::<u40,BinarySearch<u40>>(&mut result);
        },
        "uniform" => {
                let mut result = BufWriter::new(OpenOptions::new()
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open("stats_uniform.txt").unwrap());

                // Messen für STree<u40>
                //measure_uniform::<u40,STree<u40>>(&mut result);

                // Messen für BinarySearch<u40> (Baseline)
                measure_uniform::<u40,BinarySearch<u40>>(&mut result);
        }
        _ => {
            println!("Bitte verwende {} <normal_komplett|normal_viertel|uniform>",args[0]);
        }
    };
    
    println!("Ausführungsdauer {}", now.elapsed().as_secs());
    
}

/// Diese Methode dient der Hauptspeichermessung der new()-Methode verschiedener zu untersuchender Datenstrukturen E
/// mit elementen E = {u40,u48,u64} .
fn measure_uniform<E: 'static + Typable + Copy + Debug + From<u64>, T: PredecessorSetStatic<E>>(result: &mut BufWriter<File>) {

    for dir in read_dir(format!("./testdata/uniform/{}/", E::TYPE)).unwrap() {
        let dir = dir.unwrap();
        let path = dir.path();
        println!("{:?}",path);

        let mut reg = Region::new(&GLOBAL);
        
        //let values = read_from_file(path.to_str().unwrap()).unwrap();
        //let len = values.len();
        let x = vec![u40::from(0);4194304];
        let len = x.len();

        //let x = T::new(values);
        let change = reg.change_and_reset();

        // Das Ergebnis wird in die stats.txt geschrieben, die von SQLPlots analysiert und geplottet werden kann
        writeln!(result, "RESULT data_structure={}_uniform method=new size={} build_size_bytes={} size_bytes={}",T::TYPE,len,change.bytes_max_used,change.bytes_current_used).unwrap(); 
    }
}

/// Diese Methode dient der Hauptspeichermessung der new()-Methode verschiedener zu untersuchender Datenstrukturen E
/// mit elementen E = {u40,u48,u64} .
fn measure_normal_viertel<E: 'static + Typable + Copy + Debug + From<u64>, T: PredecessorSetStatic<E>>(result: &mut BufWriter<File>) {

    for dir in read_dir(format!("./testdata/normal/bereich_viertel/{}/", E::TYPE)).unwrap() {
        let dir = dir.unwrap();
        let path = dir.path();
        println!("{:?}",path);

        let mut reg = Region::new(&GLOBAL);
        let values = read_from_file(path.to_str().unwrap()).unwrap();
        let len = values.len();

 
        let x = T::new(values);
        let change = reg.change_and_reset();

        // Das Ergebnis wird in die stats.txt geschrieben, die von SQLPlots analysiert und geplottet werden kann
        writeln!(result, "RESULT data_structure={}_normal_viertel method=new size={} build_size_bytes={} size_bytes={}",T::TYPE,len,change.bytes_max_used,change.bytes_current_used + std::mem::size_of_val(&x) ).unwrap(); 
    }
}

/// Diese Methode dient der Hauptspeichermessung der new()-Methode verschiedener zu untersuchender Datenstrukturen E
/// mit elementen E = {u40,u48,u64} .
fn measure_normal_komplett<E: 'static + Typable + Copy + Debug + From<u64>, T: PredecessorSetStatic<E>>(result: &mut BufWriter<File>) {

    for dir in read_dir(format!("./testdata/normal/bereich_komplett/{}/", E::TYPE)).unwrap() {
        let dir = dir.unwrap();
        let path = dir.path();
        println!("{:?}",path);

        let mut reg = Region::new(&GLOBAL);
        let values = read_from_file(path.to_str().unwrap()).unwrap();
        let len = values.len();

 
        let x = T::new(values);
        let change = reg.change_and_reset();

        // Das Ergebnis wird in die stats.txt geschrieben, die von SQLPlots analysiert und geplottet werden kann
        writeln!(result, "RESULT data_structure={}_normal_komplett method=new size={} build_size_bytes={} size_bytes={}",T::TYPE,len,change.bytes_max_used,change.bytes_current_used + std::mem::size_of_val(&x) ).unwrap(); 
    }
}

pub fn read_from_file<T: Typable + From<u64> + Copy>(name: &str) -> std::io::Result<Box<[T]>> {
    let mut input = File::open(name)?;
    let mut lenv = Vec::new();
    std::io::Read::by_ref(&mut input).take(std::mem::size_of::<usize>() as u64).read_to_end(&mut lenv)?;
    let mut len: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
    for (i,b) in lenv.iter().enumerate() {
        len[i] = *b;
    }
    let len: usize = usize::from_le_bytes(len);

    assert!(len == (std::fs::metadata(name)?.len() as usize - std::mem::size_of::<usize>())/ std::mem::size_of::<T>());

    let mut values: Vec<T> = Vec::with_capacity(len);
    while values.len() != len {
        let mut buffer = Vec::with_capacity(std::mem::size_of::<T>());
        std::io::Read::by_ref(&mut input).take(std::mem::size_of::<T>() as u64).read_to_end(&mut buffer)?;
        let mut next_value: u64 = 0;
        for i in 0..buffer.len() {
            next_value |= (buffer[i] as u64) << (8*i);
        }

        values.push(T::from(next_value));
    }
    Ok(values.into_boxed_slice())
}