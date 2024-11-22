use crate::helpers::helper::get_writer;
use log::info;
use std::collections::HashSet;

use std::io;
use std::io::Write;

/// Write the meta file
pub fn write_meta(
    data: &Vec<(usize, usize, &HashSet<usize>, (Vec<usize>, Vec<usize>))>,
    filename: &str,
) -> io::Result<()> {
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
    data: &Vec<(usize, usize, &HashSet<usize>, (Vec<usize>, Vec<usize>))>,
    filename: &str,
) -> io::Result<()> {
    let max_len = data.iter().map(|n| n.3 .1.len()).max().unwrap();

    let mut f = get_writer(filename)?;

    let header = make_header(max_len);
    writeln!(f, "{}", header).expect("Not able to write");
    for (size, run, _ha, data) in data.iter() {
        let mut y = data
            .0
            .iter()
            .collect::<Vec<&usize>>()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        writeln!(f, "{}\t{}\tN\t{}", size, run, fillerback(&mut y, max_len),)
            .expect("Not able to write");
    }

    for (size, run, _ha, data) in data.iter() {
        let mut y2 = data
            .1
            .iter()
            .collect::<Vec<&usize>>()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        writeln!(f, "{}\t{}\tS\t{}", size, run, fillerback(&mut y2, max_len))
            .expect("Not able to write");
    }

    Ok(())
}

/// Create header for normal bootstrap output
pub fn make_header(max_len: usize) -> String {
    let mut a = String::from("Size\tRun\tType\t");
    a += (1..max_len + 1)
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join("\t")
        .as_str();
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
