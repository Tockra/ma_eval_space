extern crate stats_alloc;
extern crate serde;
extern crate rmp_serde as rmps;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use std::alloc::System;

use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};








#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;
use std::collections::HashMap;
use boomphf::Mphf;

fn main() {
    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
    //    .open("stats_hashmap_size.txt").unwrap());
        .open("stats_mphf_size.txt").unwrap());
    //measure::<u40,STree<u40>>(&mut result);

    //measure::<u40,BinarySearch>(&mut result);

    
    // Used here to ensure that the value is not
    // dropped before we check the statistics

    for i in 1..1024 {
        let keys = (0..i).collect();
        let mut reg = Region::new(&GLOBAL);
        let h = Mphf::new_parallel(2.0, &keys, None);
        //let h: HashMap<u16,usize> = HashMap::with_capacity(i as usize);
        let change = reg.change_and_reset();
        
        let build_size = change.bytes_max_used + std::mem::size_of_val(&h);
        let final_size = change.bytes_current_used + std::mem::size_of_val(&h); // Die gespeicherten Elemente abziehen
        
        writeln!(result, "RESULT data_structure=Mphf-u16,usize- method=new size={} build_size_bytes={} size_bytes={}",i,build_size,final_size ).unwrap(); 
        //writeln!(result, "RESULT data_structure=HashMap-u16,usize- method=new size={} build_size_bytes={} size_bytes={}",i,build_size,final_size ).unwrap(); 
    
    }
}


mod bench_data {
    use uint::u40;
    use ma_titan::internal::PredecessorSetStatic;

    // Todo Generics
    type Int = u40;

    #[derive(Clone)]
    pub struct BinarySearch {
        element_list: Box<[Int]>
    }

    impl PredecessorSetStatic<Int> for BinarySearch {
        fn new(elements: Vec<Int>) -> Self {
            Self {
                element_list: elements.into_boxed_slice(),
            }
        }

        fn predecessor(&self,number: Int) -> Option<Int> {
            if self.element_list.len() == 0 {
                None
            } else {
                self.pred(number, 0, self.element_list.len()-1)
            }
        }

        fn successor(&self,number: Int) -> Option<Int>{
            if self.element_list.len() == 0 {
                None
            } else {
                self.succ(number, 0, self.element_list.len()-1)
            }
        }
        
        fn minimum(&self) -> Option<Int>{
            if self.element_list.len() == 0 {
                None
            } else {
                Some(self.element_list[0])
            }
        }

        fn maximum(&self) -> Option<Int>{
            if self.element_list.len() == 0 {
                None
            } else {
                Some(self.element_list[self.element_list.len()-1])
            }
        }

        fn contains(&self, number: Int) -> bool {
            self.element_list.contains(&number)
        }

        const TYPE: &'static str = "BinarySearch";
    }

    impl BinarySearch {
        fn succ(&self, element: Int, l: usize, r: usize) -> Option<Int> {
            let mut l = l;
            let mut r = r;

            while r != l {
                let m = (l+r)/2;
                if self.element_list[m] > element {
                    r = m;
                } else {
                    l = m+1;
                }
            }
            if self.element_list[l] >= element {
                Some(self.element_list[l])
            } else {
                None
            }
        }

        fn pred(&self, element: Int, l: usize, r: usize) -> Option<Int> {
            let mut l = l;
            let mut r = r;

            while l != r {
                let m = (l+r)/2;
                if self.element_list[m] < element {
                    r = m
                } else {
                    l = m+1;
                }
            }
    
            if element >= self.element_list[l] {
                Some(self.element_list[l])
            } else {
                None
            }
        }


    }

}