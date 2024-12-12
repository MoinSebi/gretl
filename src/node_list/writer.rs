use crate::helpers::helper::get_writer;
use gfa_reader::Gfa;
use std::error::Error;
use std::io::Write;

pub fn write_wrapper(
    data: &[[u32; 4]],
    feature: &Vec<bool>,
    mask: &Vec<bool>,
    header: Vec<&str>,
    filename: &str,
    graph: &Gfa<u32, (), ()>,
) -> Result<(), Box<dyn Error>> {

    // Get writer
    let mut writer_test = get_writer(filename)?;

    // Write header
    let mut f1 = vec![String::from("Node_id")];
    f1.extend(
        feature
            .iter()
            .zip(header.iter())
            .filter(|(x, _)| **x)
            .map(|(_, x)| x.to_string())
            .collect::<Vec<String>>(),
    );
    let f2: String = f1.join("\t");
    writeln!(writer_test, "{}", f2);

    let mut c = 0;
    for (i, x) in data.iter().enumerate() {
        if mask[i] {
            let f1: Vec<String> = feature
                .iter()
                .zip(x.iter())
                .filter(|(x, _)| **x)
                .map(|(_, x)| x.to_string())
                .collect();
            let f2: String = graph.segments[c].id.to_string() + "\t" + &f1.join("\t");
            writeln!(writer_test, "{}", f2);
            c += 1;
        }
    }
    Ok(())
}
