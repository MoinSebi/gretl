use gfa_reader::{GraphWrapper, NCGfa, NCPath};

///
pub fn pan_genome(gwrapper: &GraphWrapper<NCPath>, graph: &NCGfa<()>, stats: &Vec<u32>) -> (Vec<(usize, usize)>, Vec<(String, usize, usize)>){
    eprintln!("Running core analysis");



    // Get additional information for private nodes
    let mut private_only: Vec<(String, usize, usize)> = Vec::new();


    // Iterate over each path, then summarize the sequence and nodes which is only level 1
    for path in gwrapper.genomes.iter(){
        let mut nodes = 0;
        let mut seq = 0;
        for x in path.1.iter() {
            for node in x.nodes.iter() {
                let level = stats[*node as usize - 1].clone() as usize;
                if level == 1 {
                    nodes += 1;
                    seq += graph.nodes[*node as usize -1].seq.len();
                }
            }
        }
        private_only.push((path.0.clone(), nodes, seq));
    }

    // Iterate over the data set (e.g. similarity) and summarize the sequence and nodes for each node
    let max_value = stats.iter().max().unwrap();
    let mut similarity_level: Vec<(usize, usize)> = vec![(0, 0); *max_value as usize + 1];
    for (i, x) in stats.iter().enumerate() {
        similarity_level[*x as usize].0 += 1;
        similarity_level[*x as usize].1 += graph.nodes[i].seq.len();
    }




    // Check if both values are the same (should be)
    let total_sum: usize = private_only.iter().map(|n| n.2).sum();
    if total_sum == similarity_level.get(1).unwrap().1.clone() as usize{
        eprintln!("Statistic is fine")
    }
    return (similarity_level, private_only)
}