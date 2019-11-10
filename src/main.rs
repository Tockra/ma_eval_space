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
            let item = 0_u64;
                for i in 1..u16::max_value() {
                    let i = i as usize;
                    if i % 1 == 0 {
                        let keys: Vec<u16> = (0..i as u16).collect();
                        {
                        let reg = Region::new(&GLOBAL);
                
                        let mut h = std::collections::HashMap::with_capacity(i+1);
                        for k in keys.into_iter() {
                            h.insert(k, item);
                        }

                        h.shrink_to_fit();
        
    
                        let change = reg.change();

                        
                        let build_size = change.bytes_max_used;
                        
                        let final_size = change.bytes_current_used + std::mem::size_of_val(&h); // Die gespeicherten Elemente abziehen

                        writeln!(result, "RESULT data_structure=Brown_Hash-u16,usize- method=new size={} build_size_bytes={} bit_per_element={}",i,build_size,((final_size as f64)/(i as f64)) * 8. ).unwrap();
                        // 32 da ein extra Array vorhanden ist das 16 Byte braucht (8 Zeiger, 8 Len) + Box (8). Außerdem muss nun Object auch in einer Box leigen + 8
                        writeln!(result, "RESULT data_structure=Vektor method=new size={} build_size_bytes=0 bit_per_element={}",i,(32.+(i as f64) * 2. + (i as f64) + std::mem::size_of_val(&item) as f64)/(i as f64) *8. ).unwrap(); 
                    }
                    }
                }

        },
        "mphf" => {
            let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open("stats_mphf_size.txt").unwrap());

                let h = Mphf::new_parallel(1.7, &(0..100).collect(), None);
                println!("{}", std::mem::size_of_val(&h));
                std::thread::sleep_ms(3000);
            

                for i in 1..2048 {
                    let i = i as usize;
                    if i % 1 == 0 {
                        let keys = (0..i as u16).collect();
                        {
                        let reg = Region::new(&GLOBAL);
                
                        let h = Mphf::new_parallel(1.7, &keys, None);

  
                        let change = reg.change();
        
                        
                        let build_size = change.bytes_max_used;
                        
                        let final_size = change.bytes_current_used + std::mem::size_of_val(&h); // Die gespeicherten Elemente abziehen

                        writeln!(result, "RESULT data_structure=Mphf-u16,usize- method=new size={} build_size_bytes={} bit_per_element={}",i,build_size,((final_size as f64)/(i as f64)) * 8.).unwrap();
                        // 32 da ein extra Array vorhanden ist das 16 Byte braucht (8 Zeiger, 8 Len) + Box (8). Außerdem muss nun Object auch in einer Box leigen + 8
                        writeln!(result, "RESULT data_structure=Vektor method=new size={} build_size_bytes=0 bit_per_element={}",i,(32.+(i as f64) * 2.)/(i as f64) *8. ).unwrap(); 
                    }
                    }
                
                }
        }
        _ => {
            println!("Bitte verwende {} <hashmap|mphf>",args[0]);
        }
    }
   
}