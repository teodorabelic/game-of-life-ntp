# Napredne tehnike programiranja
# Celularni automati – Game of Life  
**HPC projektni zadatak (Python + Rust)**

## 1. Opis problema
Celularni automati predstavljaju modele u kojima se prostor prikazuje kao diskretna mreža ćelija. Svaka ćelija ima stanje (aktivna ili neaktivna), a stanje se menja u diskretnim vremenskim koracima na osnovu unapred definisanih pravila.

U ovom projektu implementira se **Conway's Game of Life**, jedan od najpoznatijih i najistraženijih sistema celularnih automata. Pravila evolucije su:
1. Živa ćelija sa manje od 2 živa suseda → **umire** (usamljenost)  
2. Živa ćelija sa 2 ili 3 živa suseda → **preživljava**  
3. Živa ćelija sa više od 3 živa suseda → **umire** (prenaseljenost)  
4. Mrtva ćelija sa tačno 3 živa suseda → **postaje živa** (reprodukcija)

Cilj projekta je implementirati:
- sekvencijalnu i paralelizovanu verziju simulacije u **Python-u** i **Rust-u**
- eksperimente **jakog i slabog skaliranja**
- vizualizaciju evolucije sistema po iteracijama (Rust)

Projektni zadatak spada u kategoriju **predefinisanih HPC tema**.


## 2. Plan implementacije
### 2.1 Python implementacija (25 poena)
#### Sekvencijalna verzija
- Reprezentacija mreže kao 2D matrice (`numpy` ili standardne liste)
- Izračunavanje sledećeg stanja na osnovu suseda
- Upis stanja po iteracijama u datoteke
#### Paralelizovana verzija (multiprocessing)
- Podela matrice po redovima ili blokovima
- Svaki proces računa svoju sekciju matrice
- Sinhronizacija ivica (halo exchange)
- Spajanje delova u finalnu matricu po iteraciji

### 2.2 Rust implementacija (26 poena)
#### Sekvencijalna verzija
- Efikasan prikaz matrice pomoću `Vec<Vec<u8>>`
- Double-buffer pristup (current → next)
- Upis rezultata po iteraciji
#### Paralelizovana verzija (threads)
- Ručna podela matrice na segmente
- Korišćenje:
  - `std::thread`
  - `Arc`
  - `Mutex` ili `RwLock`
- Sinhronizacija rubnih redova
- Optimizovana raspodela posla po jezgrima procesora

## 3. Eksperimenti skaliranja
### 3.1 Jako skaliranje (strong scaling)
- Fiksna veličina problema (npr. 2000×2000)
- Variranje broja niti/procesa
- Poređenje:
  - sekvencijalna Python vs. paralelna Python
  - sekvencijalni Rust vs. paralelni Rust
Biće generisano:
- grafik jakog skaliranja (Python + Amdal)
- grafik jakog skaliranja (Rust + Amdal)
### 3.2 Slabo skaliranje (weak scaling)
- Povećanje matrice proporcionalno broju jezgara
- Posao po jezgri ostaje konstantan
- Poređenje rezultata sa Gustafsonovim zakonom
Biće generisano:
- grafik slabog skaliranja (Python)
- grafik slabog skaliranja (Rust)
Sve kombinacije parametara izvodiće se **30 puta** radi statističke tačnosti  
(srednja vrednost, standardna devijacija, identifikacija outliera).

## 4. Vizualizacija (10 poena)
Vizualizacija će prikazivati stanje Game of Life mreže po iteracijama.

Planirana Rust biblioteka:
- **Plotters** (`plotters` ili `plotters-svg`)

Podržani prikazi:
- žive i mrtve ćelije (2D prikaz)
- evolucija kroz vreme
- vizualizacija poznatih obrazaca:
  - glider
  - oscillator
  - still life
  - spaceship

## 5. Arhitektura i tehnologije
### Rust:
- `std::thread`
- `Arc`, `Mutex` / `RwLock`
- `plotters` (vizualizacija)
- opcioni `rayon` (samo ako asistent odobri)
### Python:
- `multiprocessing`
- `numpy` (opciono)

### Struktura repozitorijuma
```txt
/python
    seq_game_of_life.py
    parallel_game_of_life.py
    /outputs

/rust
    src/seq.rs
    src/parallel.rs
    /outputs

/visualization
    rust_visualizer.rs

README.md
