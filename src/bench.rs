use std::fs::File;
use std::fmt::Debug;
use std::io::{BufWriter};
use std::fs::read_dir;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::time::Instant;

use uint::{Typable};
use ma_titan::default::immutable::{Int, Pointer,STree};
use ma_titan::internal::Splittable;
use vebtrees::VEBTree as vs;


use stats_alloc::Region;


use std::io::{Write, Read};


use super::GLOBAL;

/// Diese Methode dient der Hauptspeichermessung der new()-Methode verschiedener zu untersuchender Datenstrukturen E
/// mit elementen T = {u40,u48,u64} . Diese Methode ist generisch und kann die normal-daten, die BTW-Run-Daten und gleichverteilte Daten einlesen.
pub fn measure<T: Typable + From<u64> + Copy + Debug, E: PredecessorSetStatic<T>>(data: &str, var: &str) {
    println!("Starte Speicherplatzmessung. Datenstruktur: {}, Datentyp {}, Datensatz: {}", E::TYPE, T::TYPE, data);

    let now = Instant::now();
    std::fs::create_dir_all(format!("./output/{}/", T::TYPE)).unwrap();
    let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(format!("./output/{}/{}_{}.txt", T::TYPE,E::TYPE, data.replace("/", "_"))).unwrap());

    for dir in read_dir(format!("./testdata/{}/{}/",data, T::TYPE)).unwrap() {
        let path = dir.unwrap().path();
        if path.to_str().unwrap().contains("git") {
            continue;
        }

        let i: u32 = path.to_str().unwrap().split('^').skip(1).next().unwrap().split('.').next().unwrap().parse().unwrap();

        if var == "1" {
            if i > 28 {
                continue;
            }
        } else {
                if i != 29 {
                continue;
            }
        }
        println!("{:?}",path);

        let mut reg = Region::new(&GLOBAL);
        
        let values = read_from_file(path.to_str().unwrap()).unwrap();
        let len = values.len();
        

        let _x = E::new(values);
        let change = reg.change_and_reset();

        // Das Ergebnis wird in die stats.txt geschrieben, die von SQLPlots analysiert und geplottet werden kann
        writeln!(result, "RESULT data_structure={}_{} method=new size={} build_size_bytes={} size_bytes={}",E::TYPE, data.replace("/", "_"),len,change.bytes_max_used,change.bytes_current_used).unwrap(); 
        result.flush().unwrap();
    }

    println!("Messung beendet. Dauer {} Sekunden", now.elapsed().as_secs());
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

#[derive(Clone,Debug, PartialEq, Eq)]
pub struct VEBTree {
    veb_tree: vs<usize>
}

impl<T: Int> PredecessorSetStatic<T> for VEBTree {
    const TYPE: &'static str = "vEB-Tree";

    fn new(elements: Box<[T]>) -> Self {
        let mut vtree = vs::with_capacity((elements[elements.len()-1]).into() as usize);
        for &elem in elements.iter() {
            vtree.insert((elem.into()) as usize);
        }
        Self {
            veb_tree: vtree,
        }
    }

    fn predecessor(&self,number: T) -> Option<T> {
        self.veb_tree.findprev((number.into()) as usize).and_then(|x| Some(T::new(x as u64)))
    }

    fn successor(&self,number: T) -> Option<T> {
        self.veb_tree.findnext((number.into()) as usize).and_then(|x| Some(T::new(x as u64)))
    }

    fn minimum(&self) -> Option<T> {
        self.veb_tree.minimum().and_then(|x| Some(T::new(x as u64)))
    }

    fn maximum(&self) -> Option<T> {
        self.veb_tree.maximum().and_then(|x| Some(T::new(x as u64)))
    } 

    fn contains(&self, number: T) -> bool {
        self.veb_tree.contains((number.into()) as usize)
    }
}

#[derive(Clone)]
pub struct BinarySearch<T> {
    element_list: Box<[T]>
}

impl<T: Int>  PredecessorSetStatic<T> for BinarySearch<T> {
    fn new(elements: Box<[T]>) -> Self {
        Self {
            element_list: elements,
        }
    }

    fn predecessor(&self,number: T) -> Option<T> {
        if self.element_list.len() == 0 {
            None
        } else {
            self.pred(number, 0, self.element_list.len()-1)
        }
    }

    fn successor(&self,number: T) -> Option<T>{
        if self.element_list.len() == 0 {
            None
        } else {
            self.succ(number, 0, self.element_list.len()-1)
        }
    }
    
    fn minimum(&self) -> Option<T>{
        if self.element_list.len() == 0 {
            None
        } else {
            Some(self.element_list[0])
        }
    }

    fn maximum(&self) -> Option<T>{
        if self.element_list.len() == 0 {
            None
        } else {
            Some(self.element_list[self.element_list.len()-1])
        }
    }

    fn contains(&self, number: T) -> bool {
        self.element_list.contains(&number)
    }

    const TYPE: &'static str = "BinarySearch";
}

impl<T: Int> BinarySearch<T> {
    fn succ(&self, element: T, l: usize, r: usize) -> Option<T> {
        let mut l = l;
        let mut r = r;

        if element >= self.element_list[r] {
            return None;
        }

        while r != l && element >= self.element_list[l]  {
            let m = (l+r)/2;
            if element >= self.element_list[m] {
                l = m+1;
            } else {
                r = m;
            }
        }
        if element < self.element_list[l] {
            Some(self.element_list[l])
        } else {
            None
        }
    }

    fn pred(&self, element: T, l: usize, r: usize) -> Option<T> {
        let mut l = l;
        let mut r = r;

        if element <= self.element_list[l] {
            return None;
        }

        while l != r && element <= self.element_list[r] {
            let m = (l+r)/2;
            if self.element_list[m] >= element {
                r = m-1;
            } else {
                l = m;
            }
        }

        if element > self.element_list[r] {
            Some(self.element_list[r])
        } else {
            None
        }
    }


}

pub trait PredecessorSetStatic<T> {
    fn new(elements: Box<[T]>) -> Self;
    fn predecessor(&self,number: T) -> Option<T>;
    fn successor(&self,number: T) -> Option<T>; // Optional
    fn minimum(&self) -> Option<T>;
    fn maximum(&self) -> Option<T>; 
    fn contains(&self, number: T) -> bool;

    const TYPE: &'static str;
}

impl<T: Int> PredecessorSetStatic<T> for STree<T> {
    const TYPE: &'static str = "STree";

    fn new(elements: Box<[T]>) -> Self {
         STree::<T>::new(elements)
    }

    fn predecessor(&self,number: T) -> Option<T> {
        self.locate_or_pred(number).and_then(|x| Some(self.element_list[x]))
    }

    fn successor(&self,number: T) -> Option<T> {
        self.locate_or_succ(number).and_then(|x| Some(self.element_list[x]))
    }

    fn minimum(&self) -> Option<T> {
        self.minimum()
    }

    fn maximum(&self) -> Option<T> {
        self.maximum()
    } 

    fn contains(&self, number: T) -> bool {
        let (i,j,k) = Splittable::split_integer_down(&number);
        if self.root_table[i].is_null()  {
            return false;
        }

        match self.root_table[i].get() {
            Pointer::Level(l) => {
                let l3_level = (*l).try_get(j);
                if l3_level.is_none() {
                    return false;
                } else {
                    let elem_index = match l3_level.unwrap().get() {
                        Pointer::Level(l) => {
                            (*l).try_get(k)
                        },
                        Pointer::Element(e) => {
                            Some(&*e)
                        }
                    };
                    
                        
                    if elem_index.is_none() {
                        false
                    } else {
                        self.element_list[*elem_index.unwrap()] == number
                    }
                }
                
            },

            Pointer::Element(e) => {
                self.element_list[*e] == number
            }
        }
    }
}

impl<T: Int>  PredecessorSetStatic<T> for BTreeMap<T,T> {
    fn new(elements: Box<[T]>) -> Self {
        let mut n: BTreeMap<T,T> = BTreeMap::new();
        for i in elements.iter() {
            n.insert(*i,*i);
        }
        n
    }

    fn predecessor(&self,number: T) -> Option<T> {
        self.range(T::from(0)..number).last().map(|x| *x.0)
    }

    fn successor(&self,number: T) -> Option<T>{
        self.range(number..).next().map(|x| *x.0)
    }
    
    fn minimum(&self) -> Option<T>{
        self.range(T::from(0)..).next().map(|x| *x.0)
    }

    fn maximum(&self) -> Option<T>{
        self.range(T::from(0)..).rev().next().map(|x| *x.0)
    }

    fn contains(&self, number: T) -> bool {
        self.contains_key(&number)
    }

    const TYPE: &'static str = "B-Baum";
}

