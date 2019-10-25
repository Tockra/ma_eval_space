use std::fs::File;
use std::fmt::Debug;
use std::io::{BufWriter};
use std::fs::read_dir;
use std::fs::OpenOptions;
use std::time::Instant;

use uint::{Typable};
use ma_titan::default::immutable::{Int,STree};


use stats_alloc::Region;


use std::io::{Write, Read};


use super::GLOBAL;

/// Diese Methode dient der Hauptspeichermessung der new()-Methode verschiedener zu untersuchender Datenstrukturen E
/// mit elementen T = {u40,u48,u64} . Diese Methode ist generisch und kann die normal-daten, die BTW-Run-Daten und gleichverteilte Daten einlesen.
pub fn measure<T: Typable + Int + From<u64> + Copy + Debug>(data: &str, var: u32, name: &str) {
    println!("Starte Speicherplatzmessung. Datenstruktur: STree, Datentyp {}, Datensatz: {}", T::TYPE, data);

    let now = Instant::now();
    std::fs::create_dir_all(format!("./output/{}/", T::TYPE)).unwrap();
    let mut result = BufWriter::new(OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(format!("./output/{}/STree_{}_{}_{}.txt", T::TYPE, name, data.replace("/", "_"), var)).unwrap());

    for dir in read_dir(format!("./testdata/{}/{}/",data, T::TYPE)).unwrap() {
        let path = dir.unwrap().path();
        if path.to_str().unwrap().contains("git") {
            continue;
        }

        if data != "bwt_runs" {
            let i: u32 = path.to_str().unwrap().split('^').skip(1).next().unwrap().split('.').next().unwrap().parse().unwrap();

            if i != var {
                continue;
            }
        } else {
            if !path.to_str().unwrap().contains(var.to_string().as_str()) {
                continue;
            }
        }
    
        println!("{:?}",path);
        
        // Keine Ahnung ob das wirklich nötig ist, aber zur Sicherheit!
        std::thread::sleep(std::time::Duration::from_millis(3000));
        let mut reg = Region::new(&GLOBAL);
        
        let values = read_from_file::<T>(path.to_str().unwrap()).unwrap();
        let len = values.len();
        

        let x = STree::<T>::new(GLOBAL,values);
        let change = reg.change_and_reset();

        // Das Ergebnis wird in die stats.txt geschrieben, die von SQLPlots analysiert und geplottet werden kann
        writeln!(result, "RESULT data_structure=STree_{} method=new size={} build_size_bytes={} size_bytes={} hash_tables_bytes={} levelcount={} number_keys={}", name,len,change.bytes_max_used
                ,change.bytes_current_used, x.hash_maps_in_bytes, x.level_count, x.number_of_keys).unwrap(); 
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
