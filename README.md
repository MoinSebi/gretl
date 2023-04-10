# gfastats
#### Description 
Small tool for basic graph statistics using GFA format.   
  


#### Installation: 

**Git**  
```
git clone https://github.com/MoinSebi/gfastats  
cd gfastats   
cargo build --release  
./target/release/gfastats  
```

**Cargo** 
```
cargo install gfastats
```

#### Usage

##### ID2INT
Convert any string as ID to u32 integer. Do not use when graph is already digits only + some statistics might not be working as intended :) 