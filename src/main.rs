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

             
                    writeln!(result, "RESULT data_structure=HashMap-u16,usize- method=new size={} build_size_bytes={} size_bytes={}",i,build_size,final_size).unwrap(); 
                }

        },
        "mphf" => {
            let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open("stats_mphf_size.txt").unwrap());
                
                for i in 1..1024 {
                    let i = i as usize;
                    if i % 10 == 0 {
                        let keys = (0..i).collect();
                        
                        let mut reg = Region::new(&GLOBAL);
                
                        let h = Mphf::new_parallel(1.7, &keys, None);

  
                        let change = reg.change();
        
                        
                        let build_size = change.bytes_allocated - change.bytes_deallocated;
                        
                        let final_size = change.bytes_current_used; // Die gespeicherten Elemente abziehen
                        
                        writeln!(result, "RESULT data_structure=Mphf-u16,usize- method=new size={} build_size_bytes={} size_bytes={}",i,build_size,((final_size as f64)/(i as f64)) * 8. ).unwrap(); 
                    
                    }
                
                }
        }
        _ => {
            println!("Bitte verwende {} <hashmap|mphf>",args[0]);
        }
    }
   
}