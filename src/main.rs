extern crate stats_alloc;
extern crate serde;
extern crate rmp_serde as rmps;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use std::alloc::System;

use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::env;


#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;
use std::collections::HashMap;
use boomphf::Mphf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Bitte gebe an, welche Hashfunktion du evaluieren möchtest!");
    }

    match args[1].as_ref() {
        "hashmap" => {
            let mut result = BufWriter::new(OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open("stats_hashmap_size.txt").unwrap());

                for i in 1..1024 {
                    let mut reg = Region::new(&GLOBAL);
                    let change;

                    let h: HashMap<u16,usize> = HashMap::with_capacity(i as usize);
                    change = reg.change_and_reset();
                    
                    let build_size = change.bytes_max_used;
                    let final_size = change.bytes_current_used; // Die gespeicherten Elemente abziehen

                    let x = vec![0u16;i].into_boxed_slice();
                    let change = reg.change_and_reset();
                    let build_size_base = change.bytes_max_used;
                    let final_size_base = change.bytes_current_used;
             
                    writeln!(result, "RESULT data_structure=HashMap-u16,usize- method=new size={} build_size_bytes={} size_bytes={}",i,build_size,final_size).unwrap(); 
                    writeln!(result, "RESULT data_structure=Base method=new size={} build_size_bytes={} size_bytes={}",i,build_size_base,final_size_base ).unwrap(); 
                }

        },
        "mphf" => {
            let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open("stats_mphf_size.txt").unwrap());

                for i in 1..u16::max_value() {
                    let i = i as usize;
                    if i % 1000 == 1 {
                        let keys = (0..i).collect();
                        let mut reg = Region::new(&GLOBAL);
                        let change;
                
                        let h = Mphf::new_parallel(2.0, &keys, None);
                        change = reg.change_and_reset();
        
                        
                        let build_size = change.bytes_max_used;
                        let final_size = change.bytes_current_used; // Die gespeicherten Elemente abziehen
                        
                    
                        let x = vec![0u16;i].into_boxed_slice();
                        let change = reg.change_and_reset();
                        let build_size_base = change.bytes_max_used;
                        let final_size_base = change.bytes_current_used;
                        writeln!(result, "RESULT data_structure=Mphf-u16,usize- method=new size={} build_size_bytes={} size_bytes={}",i,build_size,(final_size as f64)/(i as f64) ).unwrap(); 
                        writeln!(result, "RESULT data_structure=Base method=new size={} build_size_bytes={} size_bytes={}",i,build_size_base,final_size_base ).unwrap(); 
                    
                    }
                
                }
        }
        _ => {
            println!("Bitte verwende {} <hashmap|mphf>",args[0]);
        }
    }
   
}