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

use bench_data::BinarySearch;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    let mut result = BufWriter::new(OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("stats.txt").unwrap());
    measure::<u40,STree<u40>>(&mut result);

    measure::<u40,BinarySearch>(&mut result);

    
    // Used here to ensure that the value is not
    // dropped before we check the statistics
    
}

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

        writeln!(result, "RESULT data_structure={} method=new size={} build_size_bytes={} size_bytes={}",T::TYPE,len,change.bytes_max_used,change.bytes_current_used + std::mem::size_of_val(&x) ).unwrap(); 
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