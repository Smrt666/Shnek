# Shnek
A simple 3D snake game in Rust.

## Opis projekta
Ta projekt je preprosta 3D igra kača, napisana v programskem jeziku Rust. 
Cilj igre je voditi kačo skozi tridimenzionalni prostor, zbirati hrano in 
se izogibati trkom z lastnim telesom. Sten ni, zato lahko kača potuje iz spodnje 
navidezne stranice prostora do zgornje in podobno v ostale smeri. Cilj je to 
tudi videti - kača je pred sabo, za sabo, levo in desno od sebe. Če bo ostal čas
se lahko doda tudi rezultate drugih igralcev, ali pa morda igranje z več ljudmi, 
različne težavnosti, ali pa bo prišla kakšna druga ideja.

## Delo na projektu
Pred pushom (ali pa vsaj pred merganjem) poskrbi, da tole pravi da je ok:
```sh
cargo fmt
cargo clippy
```

Če testiraš in je stvar laggy, lahko uporabiš optimizacije:
```sh
cargo run --profile=opt
```

Preden začneš z delom naredi iz `dev` brancha svoj branch in delaj na njem.

### Todos:
- [x] Kača
- [ ] Hrana za kačo
- [x] Player input - premikanje

Za ostalo glej issues.