use clap::ArgMatches;
use gfa_reader::{check_numeric_compact_gfafile, Gfa, Pansn};
use rayon::prelude::*;

use std::io::Write;

use log::{info, warn};

use crate::helpers::helper::{get_writer, mean};
use chrono::Local;
use hashbrown::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

/// Block main function
///
/// Easy block function
/// Extract the subpath from a graph for each node
pub fn block_main(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running 'gretl block' analysis");
    // Input
    let graph_file = matches.value_of("gfa").unwrap();
    let mut sep = matches.value_of("PanSN").unwrap_or(" ");

    // Block parameters
    // Based on nodes
    let node_window: usize = matches
        .value_of("node-window")
        .unwrap_or("1000")
        .parse::<usize>()
        .expect("Error: node-window must be an integer");
    let node_step: usize = matches
        .value_of("node-step")
        .unwrap_or("1000")
        .parse()
        .expect("Error: node-step must be an integer");

    // This does not work
    if node_step > node_window {
        panic!("Step size can't be larger than window size");
    }

    // Sequence block
    let sequence_window = matches.value_of("sequence-window");
    let sequence_step = matches.value_of("sequence-step");

    let cutoff_distance: usize = matches.value_of("distance").unwrap().parse().unwrap();

    // Output
    let output_prefix = matches.value_of("output").unwrap();
    let threads: usize = matches.value_of("threads").unwrap().parse().unwrap();

    info!("GFA file: {}", graph_file);
    info!(
        "Pan-SN: {}",
        if sep == "\n" {
            "None".to_string()
        } else {
            format!("{:?}", sep)
        }
    );
    info!("Node window.md size: {}", node_window);
    info!("Node step size: {}", node_step);
    info!("Sequence window.md size: {:?}", sequence_window);
    info!("Sequence step size: {:?}", sequence_step);
    info!("Distance: {}", cutoff_distance);
    info!(
        "Output file: {}\n",
        if output_prefix == "-" {
            "stdout"
        } else {
            output_prefix
        }
    );

    info!("Numeric check");
    let num_com = check_numeric_compact_gfafile(matches.value_of("gfa").unwrap());

    if num_com.0 {
        if !num_com.1 {
            warn!("The GFA file is not sorted.")
        }

        info!("Reading GFA file");
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file_multi(graph_file, threads);
        if graph.paths.is_empty() && sep == "\n" {
            sep = "#"
        }
        graph.walk_to_path(sep);

        info!("Number of paths: {}", graph.paths.len());
        let (min1, max1) = get_min_max(&graph);
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);

        info!("Block generation");
        if !matches.is_present("sequence-window")
            && !matches.is_present("sequence-step")
            && !matches.is_present("node-window")
            && !matches.is_present("node-step")
        {
            warn!("Running on default values for block generation. Node-step: 1000 and Node-window: 1000");
        }
        let blocks = block_wrapper(
            &graph,
            node_step,
            node_window,
            sequence_window,
            sequence_step,
        )?;
        info!("Number of blocks: {}", blocks.len());

        let node_sizes = node_size(&graph, min1, max1);

        info!("Extracting blocks");
        wrapper_blocks(
            &wrapper,
            node_sizes,
            blocks,
            cutoff_distance,
            true,
            output_prefix,
            threads,
            &graph,
        )?;
        info!("Done");
    } else {
        panic!("The node IDs in the GFA file are not numeric");
    }
    Ok(())
}

pub fn get_min_max(graph: &Gfa<u32, (), ()>) -> (u32, u32) {
    let mut min = u32::MAX;
    let mut max = 0;
    for x in graph.segments.iter() {
        if x.id < min {
            min = x.id;
        }
        if x.id > max {
            max = x.id;
        }
    }
    (min, max)
}

/// Create blocks based on nodes in the graph
///
/// Limit:
/// 1. Nodes window
/// 2. Sequence sizes
pub fn block_wrapper(
    graph: &Gfa<u32, (), ()>,
    node_step: usize,
    node_window: usize,
    sequence_window: Option<&str>,
    sequence_step: Option<&str>,
) -> Result<Vec<[u32; 2]>, Box<dyn std::error::Error>> {
    let mut blocks = Vec::new();
    if sequence_window.is_some() && sequence_step.is_some() {
        let sequence_usize = sequence_window.unwrap().parse()?;
        let sequence_window_usize = sequence_step.unwrap().parse()?;
        blocks = block_seq(graph, sequence_usize, sequence_window_usize);
    } else if node_step > node_window {
        panic!("Step size can't be larger than window size");
    } else if sequence_step.is_some() && sequence_window.is_none() {
        warn!("You need to provide a sequence if you provide a sequence window. Now Define blocks on node-step and node-window instead");
        blocks = blocks_node(graph, node_step, node_window);
    } else if sequence_window.is_some() && sequence_step.is_none() {
        warn!("You need to provide a sequence step if you provide a sequence. Now Define blocks on node-step and node-window instead");
        blocks = blocks_node(graph, node_step, node_window);
    } else {
        blocks = blocks_node(graph, node_step, node_window);
    }

    Ok(blocks)
}

/// Make blocks
///
///  - A block starts at a node and end at a node
///  - Returns start and end nodes of a block
pub fn blocks_node(graph: &Gfa<u32, (), ()>, step: usize, wsize: usize) -> Vec<[u32; 2]> {
    let mut blocks = Vec::new();
    let (_min1, _max1) = get_min_max(graph);
    let total_segments = graph.segments.len();

    for (segment_index, segment) in graph.segments.iter().enumerate().step_by(step) {
        let end = (segment_index + wsize).min(total_segments - 1);

        blocks.push([segment.id, graph.segments[end].id]);
    }
    blocks
}

/// Use the step-index to start a new block
/// Finish block if the size is larger than the sequence
pub fn block_seq(graph: &Gfa<u32, (), ()>, size: usize, step_size: usize) -> Vec<[u32; 2]> {
    let starts = get_starts(graph, step_size);

    let mut result = Vec::new();
    let mut sequence = 0;
    let mut starting_id = graph.segments[0].id;
    for step_start in starts.iter() {
        for step_index in *step_start..graph.segments.len() {
            sequence += graph.segments[step_index].sequence.get_len();
            if sequence > size {
                result.push([starting_id, graph.segments[step_index].id]);
                starting_id = graph.segments[step_index].id;
                sequence = 0;
            }
        }
    }
    result
}

/// Get the index where the window changes (by step)
pub fn get_starts(graph: &Gfa<u32, (), ()>, step: usize) -> Vec<usize> {
    let mut step_index = Vec::new();
    let mut pos = 0;
    for (index, x) in graph.segments.iter().enumerate() {
        pos += x.sequence.get_len();
        if pos > step {
            step_index.push(index);
            pos = 0;
        }
    }
    step_index
}

/// Node size index
pub fn node_size(graph: &Gfa<u32, (), ()>, min: u32, max: u32) -> Vec<usize> {
    let mut node_size = vec![0; ((max - min) + 2) as usize];

    for node in graph.segments.iter() {
        node_size[(node.id - min) as usize] = node.sequence.get_len();
    }
    node_size
}

/// Wrapper function for blocks
///
/// Iterate over blocks
/// Iterate over each path
/// Iterate over each node
/// If node is in block:
///     if block is not empty: add to block
///    else: create new block
/// else: add distance
pub fn wrapper_blocks(
    _graph2: &Pansn<u32, (), ()>,
    _node_size: Vec<usize>,
    block: Vec<[u32; 2]>,
    max_distance: usize,
    _blocks: bool,
    _out_prefix: &str,
    threads: usize,
    graph: &Gfa<u32, (), ()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (min1, _max1) = get_min_max(graph);

    let mut block_index: Vec<usize> = block
        .iter()
        .enumerate()
        .flat_map(|(i, x)| (x[0]..x[1]).map(move |_y| i))
        .collect();
    block_index.push(block.len() - 1);
    let p1 = Arc::new(Mutex::new(vec![Vec::new(); block.len()]));
    //
    let path_per_chunk = (graph.paths.len() + threads - 1) / threads;

    let atomic_count = AtomicU32::new(0);
    let paths_number = graph.paths.len();
    graph
        .paths
        .par_chunks(path_per_chunk)
        .enumerate()
        .for_each(|(index, chunk)| {
            for (genome_id, path) in chunk.iter().enumerate() {
                atomic_count.fetch_add(1, Ordering::Relaxed);
                std::io::stderr().flush().unwrap();
                eprint!(
                    "\r{}          Path {:?}/{}",
                    Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                    atomic_count.load(Ordering::Relaxed),
                    paths_number
                );

                let mut cumulative_length = 0; // Variable to track the cumulative length
                let genome_id = genome_id + index * path_per_chunk;
                let mut block_index_cumulative: Vec<(usize, usize, u32)> = path
                    .nodes
                    .iter()
                    .enumerate()
                    .map(|(i, node_id)| {
                        let node = graph.get_sequence_by_id(node_id).len() as u32; // Return None if the node isn't found
                        let block_value = block_index[(*node_id - min1) as usize];
                        cumulative_length += node; // Update cumulative length
                                                   // Get the block value
                        (block_value, i, cumulative_length - node) // Return the block value and cumulative length
                    })
                    .collect();
                block_index_cumulative.sort_by(|comp1, b| comp1.0.cmp(&b.0));

                // Start index, end index, genome id
                let mut p: Vec<Vec<[usize; 3]>> = vec![Vec::new(); block.len()];
                //info!("{:?}", [block_index_cumulative[0], block_index_cumulative[1], block_index_cumulative[2]]);
                let mut prev_block = block_index_cumulative[0].0;
                let mut prev_pos = block_index_cumulative[0].2;
                let mut start_index = block_index_cumulative[0].1;
                for index in 1..block_index_cumulative.len() {
                    let (block_value, i, pos) = &block_index_cumulative[index];
                    if *block_value != prev_block {
                        //println!("hit2 {} {} {}", start_index, i, prev_pos);
                        if !p.is_empty() {
                            p[prev_block].push([
                                start_index,
                                block_index_cumulative[index - 1].1,
                                genome_id,
                            ]);
                        }
                        prev_block = *block_value;
                        prev_pos = *pos;
                        start_index = *i;
                    } else {
                        //println!("dsakjdsa {} {} {}", *pos as usize, prev_pos as usize , graph.get_node_by_id(&(path.nodes[aa[index-1].1 as usize])).length as usize);

                        let pp = *pos as usize
                            - prev_pos as usize
                            - graph
                                .get_sequence_by_id(
                                    &(path.nodes[block_index_cumulative[index - 1].1]),
                                )
                                .len();
                        //println!("hit");
                        if pp > max_distance {
                            //println!("dsakjdsa {} {} {} {} {} {}", *pos as usize, prev_pos as usize , aa[index].1, aa[index-1].1, graph.get_node_by_id(&(path.nodes[aa[index].1 as usize])).length as usize, graph.get_node_by_id(&(path.nodes[aa[index-1].1 as usize])).length as usize);
                            p[prev_block].push([
                                start_index,
                                block_index_cumulative[index - 1].1,
                                genome_id,
                            ]);
                            prev_pos = *pos;
                            start_index = *i;
                        }

                        prev_pos = *pos;
                        prev_block = *block_value;
                    }
                }
                let p1 = Arc::clone(&p1); // Clone the Arc to share the Mutex across threads

                let mut shared_p1 = p1.lock().unwrap();
                for (i, x) in p.iter_mut().enumerate() {
                    shared_p1[i].append(x)
                }
            }
        });
    eprintln!();
    info!("Writing output");
    let _o = p1.lock().unwrap();
    assert_eq!(block.len(), _o.len());
    let combined: Vec<_> = _o.iter().zip(block.iter()).collect();
    let mut writer = get_writer(_out_prefix)?;
    writeln!(
        writer,
        "Block\t#Traversals\t#Paths\t#Samples\t#Nodes\t#Nodes (average)\tSequence [bp] (average)"
    )
    .unwrap();
    let hm1 = pansn2id(_graph2, graph);

    for traversal in combined.iter() {
        let mut s = String::new();
        let block_name = format!("{}-{}\t", traversal.1[0], traversal.1[1]);
        let trav = traversal.0.iter().len();
        let paths = traversal
            .0
            .iter()
            .map(|y| y[2])
            .collect::<HashSet<usize>>()
            .len();
        let trav1 = traversal
            .0
            .iter()
            .map(|y| &graph.paths[y[2]].nodes[y[0]..y[1]])
            .collect::<Vec<_>>();
        let samples = traversal
            .0
            .iter()
            .map(|y| hm1.get(&y[2]).unwrap())
            .collect::<HashSet<&usize>>()
            .len();
        let len1 = trav1.iter().map(|y| y.len() as u32).collect::<Vec<u32>>();
        let total_len = trav1
            .iter()
            .map(|y| {
                y.iter()
                    .map(|z| graph.get_sequence_by_id(z).len() as f64)
                    .sum::<f64>()
            })
            .collect::<Vec<f64>>();

        s.push_str(&block_name);
        s.push_str(format!("{:?}\t", trav).as_str());
        s.push_str(format!("{:?}\t", paths).as_str());
        s.push_str(format!("{:?}\t", samples).as_str());
        s.push_str(format!("{:?}\t", len1.len()).as_str());
        s.push_str(format!("{:?}\t", mean(&len1)).as_str());
        s.push_str(format!("{:?}", mean(&total_len)).as_str());

        writeln!(writer, "{}", &s);
    }

    Ok(())
}

pub fn pansn2id(pansn: &Pansn<u32, (), ()>, graph: &Gfa<u32, (), ()>) -> HashMap<usize, usize> {
    let mut hm1 = HashMap::new();
    for (id, x) in graph.paths.iter().enumerate() {
        hm1.insert(&x.name, id);
    }
    let mut hm2 = HashMap::new();
    for (id, x) in pansn.genomes.iter().enumerate() {
        for y in x.get_haplo_path() {
            hm2.insert(*hm1.get(&y.name).unwrap(), id);
        }
    }
    hm2
}
