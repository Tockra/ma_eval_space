## Verwendung
Diese Bibliothek dient den Speicherplatzmessung der Datenstrukturen aus: https://github.com/Tockra/ma_titan (ma_titan) .
Die Testdaten müssen zuvor mit: https://github.com/Tockra/ma_random_data_generator erzeugt werden und anschließend in ma_eval_space/testdata gespeichert werden.

Dabei müssen alle Ordner (ma_eval_space) entsprechend ihres Branch-Namens benannt werden. Außer: space_efficient* wird zu se* und hash_brown_hash* wird zu brown_hash* .



Das File eval-scripts/space/run-eval.sh kann wie folgt verwendet werden:
```bash
./run_eval.sh <datenstruktur={stree, rbtree, binary}> <Ordnername> <verteilung={normal,uniform}> <datentyp={u40,u48,u64}> <min={1,2,3,4,..,32}> <max={1,2,3,4,..,32}> <Pfad zu ma_eval_space> <sbatch> 
```

**<datenstruktur={stree, rbtree, binary}>:** 
- stree: die stree-Implementierung, des entsprechenden branches von ma_eval_space wird untersucht
- rbtree: Rot-Schwarz-Baum (bitte master-Branch verwenden)
- binary: Binäre Suche (bitte master-Branch verwenden)

**< Ordnername >**
- Ordnername von ma_eval_space (siehe oben)

**<verteilung={normal,uniform}>**
- uniform: Gleichverteilung
- normal: Normalverteilung

**<datentyp={u40,u48,u64}>**
- u40 : 40-Bit-Integer
- u48 : 48-Bit-Integer
- u64 : 64-Bit-Integer

**<min={1,2,3,4,..,32}>**
- Zweierpotenz, die mindestens überprüft wird (und alles zwischen Min und Max)

**<max={1,2,3,4,..,32}>**
- Zweierpotenz, die maximal überprüft wird (und alles zwischen Min und Max)

**<Pfad zu ma_eval_space>**
- /pfad/zu/ma_eval_space (mit dem Namen der in Ordnername angegeben wurde)

**< sbatch >**
- entweder sbatch hinschreiben, dann werden slurm-jobs angelegt, sonst wird die Evaluierung der Reihe nach von 2^min bis 2^max ausgeführt.
- Verwendet wird bei Laufzeitevaluierung cquad01 

### Beispiel
Die Speicherplatz für Gleichverteilte Werte zwischen 2^20 und 2^30 soll gemessen werden von binary_2 (u40):
```bash
git clone --branch=space_efficient https://github.com/Tockra/ma_eval_space/ se
cd path/to/eval-scripts/space/
./run_eval.sh stree se uniform u40 20 31 path/to/se/ sbatch # wird sbatch weggelassen, wird die Laufzeitanalyse lokal ausgeführt
``` 


## Branches
Für die verschiedenen Branches von `ma_titan` existieren hier gleichnamige Branches, in denen der entsprechende Branch von `ma_titan` verwendet wird. Folgende Auflistung gibt an, wie
die Branches den Branches der Arbeit zuzuordnen sind.


Die Branches können wie folgt zugeordnet werden:
ma_titan space_efficient_16 (u40) soll mit gleichverteilten Daten evaluiert werden. Dabei sollen die new-methoden evaluiert werden:

### 40-Bit:

| Branch               | Bezeichnung in der Arbeit (Evaluierung)|
| -------------------- |----------------------------------------| 
| master               | mphf_2                                 |
| mphf_16              | mphf_1                                 |
| original             | original_2                             | 
| original_16          | original_1                             | 
| lookup               | lookup_2                               | 
| lookup_16            | lookup_1                               | 
| threshold            | threshold_2                            | 
| threshold_16         | threshold_1                            | 
| space_efficient      | binary_2                               | 
| space_efficient_16   | binary_1                               | 
| fnv_hash             | fnv_2                                  | 
| fnv_hash_16          | fnv_1                                  | 
| hash_brown_hash      | ahash_2                                | 
| brown_hash_16        | ahash_1                                | 

### 48-Bit:

| Branch                | Bezeichnung in der Arbeit (Evaluierung)|
| --------------------- |----------------------------------------| 
| mphf_u48_1            | mphf_1                                 |
| mphf_u48_2            | mphf_2                                 |
| original_u48_1        | original_1                             | 
| original_u48_2        | original_2                             | 
| lookup_u48_1          | lookup_1                               | 
| lookup_u48_2          | lookup_2                               | 
| threshold_u48_1       | threshold_1                            | 
| threshold_u48_2       | threshold_2                            | 
| space_efficient_u48_1 |  binary_1                              | 
| space_efficient_u48_2 | binary_2                               | 
| fnv_hash_u48_1        | fnv_1                                  | 
| fnv_hash_u48_2        | fnv_2                                  | 
| brown_hash_u48_1      | ahash_1                                | 
| brown_hash_u48_2      | ahash_2                                | 

### 64-Bit:

| Branch                 | Bezeichnung in der Arbeit (Evaluierung)|
| ---------------------- |----------------------------------------| 
| mphf_u64_1             | mphf_1                                 |
| mphf_u64_2             | mphf_2                                 |
| original_u64_1         | original_1                             | 
| original_u64_2         | original_2                             | 
| lookup_u64_1           | lookup_1                               | 
| lookup_u64_2           | lookup_2                               | 
| threshold_u64_1        | threshold_1                            | 
| threshold_u64_2        | threshold_2                            | 
| space_efficient_u64_1  |  binary_1                              | 
| space_efficient_u64_2  | binary_2                               | 
| fnv_hash_u64_1         | fnv_1                                  | 
| fnv_hash_u64_2         | fnv_2                                  | 
| brown_hash_u64_1       | ahash_1                                | 
| brown_hash_u64_2       | ahash_2                                | 
