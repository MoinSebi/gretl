use crate::helpers::helper::{get_writer, print_or_write};
use log::info;
use std::collections::HashSet;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::{stdout, BufWriter, Write};

/// Write the meta file
pub fn write_meta(data: Vec<(usize, usize, HashSet<usize>)>, filename: &str) -> io::Result<()> {
    info!("Writing meta");

    let mut writer = get_writer(filename)?;

    for entry in data.iter() {
        writeln!(
            writer,
            "{}\t{}\t{}",
            entry.0,
            entry.1,
            entry
                .2
                .iter()
                .collect::<Vec<&usize>>()
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )?;
    }
    Ok(())
}

/// Write output file
pub fn write_output(
    data: Vec<(usize, usize, (Vec<usize>, Vec<usize>))>,
    filename: &str,
) -> io::Result<()> {
    info!("Writing output");
    let max_len = data.iter().map(|n| n.2 .1.len()).max().unwrap();

    let mut f = get_writer(filename)?;

    let header = make_header(max_len);
    writeln!(f, "{}", header).expect("Not able to write");
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
        writeln!(
            f,
            "{}\t{}\t{}\t{}",
            size,
            run,
            fillerback(&mut y, max_len),
            fillerback(&mut y2, max_len)
        )
        .expect("Not able to write");
    }
    Ok(())
}

/// Create header for normal bootstrap output
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
        a += "\t";
        act_len += 1;
    }
    a
}
