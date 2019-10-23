use std::fs::File;
use std::fmt::Debug;
use std::io::{BufWriter};
use std::fs::read_dir;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::time::Instant;

use uint::{Typable};
use ma_titan::default::immutable::{Int,STree};


use stats_alloc::Region;


use std::io::{Write, Read};


use super::GLOBAL;

/// Diese Methode dient der Hauptspeichermessung der new()-Methode verschiedener zu untersuchender Datenstrukturen E
/// mit elementen T = {u40,u48,u64} . Diese Methode ist generisch und kann die normal-daten, die BTW-Run-Daten und gleichverteilte Daten einlesen.
pub fn measure<T: Typable + From<u64> + Copy + Debug, E: PredecessorSetStatic<T>>(data: &str, var: &str, name: &str) {
    println!("Starte Speicherplatzmessung. Datenstruktur: {}, Datentyp {}, Datensatz: {}", E::TYPE, T::TYPE, data);

    let now = Instant::now();
    std::fs::create_dir_all(format!("./output/{}/", T::TYPE)).unwrap();
    let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(format!("./output/{}/{}_{}_{}_{}.txt", T::TYPE,E::TYPE, name, data.replace("/", "_"), var)).unwrap());

    for dir in read_dir(format!("./testdata/{}/{}/",data, T::TYPE)).unwrap() {
        let path = dir.unwrap().path();
        if path.to_str().unwrap().contains("git") {
            continue;
        }

        if data != "bwt_runs" {
            let i: u32 = path.to_str().unwrap().split('^').skip(1).next().unwrap().split('.').next().unwrap().parse().unwrap();

            if var == "1" {
                if i > 30 { 
                    continue;
                }
            } else {
                if i <= 29 {
                    continue;
                }
            }
        }
    
        println!("{:?}",path);
        
        // Keine Ahnung ob das wirklich nötig ist, aber zur Sicherheit!
        std::thread::sleep(std::time::Duration::from_millis(3000));
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

#[derive(Clone)]
pub struct RBTree<T> where T: Int + Default + num::Bounded {
    rb: treez::rb::TreeRb<T,T>
}

impl<T: Int + Default + num::Bounded> PredecessorSetStatic<T> for RBTree<T> {
    const TYPE: &'static str = "Rot-Schwarz-Baum";

    fn new(elements: Box<[T]>) -> Self {
        let mut rb = treez::rb::TreeRb::with_capacity(elements.len());
        for &elem in elements.into_iter() {
            rb.insert(elem,elem);
        }
        rb.shrink_to_fit();
        Self {
            rb: rb,
        }
    }

    fn predecessor(&self,number: T) -> Option<T> {
        self.rb.predecessor(number).map(|x| *x)
    }

    fn successor(&self,number: T) -> Option<T> {
        self.rb.successor(number).map(|x| *x)
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

    const TYPE: &'static str = "B-Baum";
}

