pub fn writer_core(data: Vec<usize>, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (d, x) in data.iter().zip(tab){
        write!(f, "{}: {}\n", x, d);
    }

}