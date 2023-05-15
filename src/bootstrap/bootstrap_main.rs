use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use clap::{ArgMatches};
use gfa_reader::Gfa;
use crate::bootstrap::helper::random_numbers;
use crate::bootstrap::meta::{combinations_maker, combinations_maker_wrapper, do_one_iteration2, make_meta, reduce_meta};
use crate::bootstrap::reader::read_meta;
use crate::bootstrap::writer::{write_meta, write_output};
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::stats::helper::get_filename;

pub fn bootstrap_main(matches: &ArgMatches){
    let graph = read_graph(matches);
    let gw = make_wrapper(&graph, matches);
    let output = &get_filename(matches.value_of("output").unwrap());

    let mut combii: Vec<(usize, usize, HashSet<usize>)> = Vec::new();
    if matches.is_present("meta input"){
        combii = read_meta(matches.value_of("meta input").unwrap());
    } else {
        combii = combinations_maker_wrapper(&4, &2);

    }
    let mut line = -1;
    if matches.is_present("meta line"){
        line = matches.value_of("meta line").unwrap().parse().unwrap();
    }
    let mut core = -1;
    if matches.is_present("level"){
        core = matches.value_of("level").unwrap().parse().unwrap();
    }
    eprintln!("Running bootstrap");
    let amount_path = gw.genomes.len();

    //let c: usize = matches.value_of("number").unwrap().parse().unwrap();
    let mut metas = Vec::new();
    let mut total = Vec::new();

    reduce_meta(& mut combii, line, core);

    for (x, i, x1) in combii.iter(){
        let k: Vec<usize> = x1.iter().cloned().collect();
        let dd = do_one_iteration2(&gw, &graph, &k, "core");
        println!("{} {:?} {:?}", x, i, dd.0);
        total.push((*x, *i, dd));
        metas.push((*x, *i, x1.clone()));
    }


    if matches.is_present("meta"){
        let metas_output = matches.value_of("meta").unwrap();
        write_meta(metas, metas_output);
    }


    write_output(total, output);







}

