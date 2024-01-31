use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Write the meta file
pub fn write_meta(data: Vec<(usize, usize, HashSet<usize>)>, filename: &str) {
    eprintln!("Writing meta");

    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter() {
        write!(
            f,
            "{}\t{}\t{}\n",
            x.0,
            x.1,
            x.2.iter()
                .collect::<Vec<&usize>>()
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
        .expect("Not able to write");
    }
}

/// Write output file
pub fn write_output(data: Vec<(usize, usize, (Vec<usize>, Vec<usize>))>, filename: &str) {
    eprintln!("Writing meta");
    println!("Writing meta {:?}", data);
    let max_len = data.iter().map(|n| n.2 .1.len()).max().unwrap();

    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let header = make_header(max_len);
    write!(f, "{}\n", header).expect("Not able to write");
    for (size, run, data) in data.iter() {
        let mut y = data
            .0
            .iter()
            .collect::<Vec<&usize>>()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        let mut y2 = data
            .1
            .iter()
            .collect::<Vec<&usize>>()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        write!(
            f,
            "{}\t{}\t{}\t{}\n",
            size,
            run,
            fillerback(&mut y, max_len),
            fillerback(&mut y2, max_len)
        )
        .expect("Not able to write");
    }
}

/// Create header for output
pub fn make_header(max_len: usize) -> String {
    let mut a = String::from("Size\tRun\t");
    for x in 0..max_len {
        a = a + &format!("Node:{}\t", x + 1);
    }
    for x in 0..max_len {
        a = a + &format!("Seq:{}\t", x + 1);
    }
    a
}

/// Fill up the vector with tabs
pub fn fillerback(stri: &mut Vec<String>, maxlen: usize) -> String {
    let mut act_len = stri.len();
    let mut a = stri.join("\t");
    while act_len < maxlen {
        a = a + "\t";
        act_len += 1;
    }
    a
}
