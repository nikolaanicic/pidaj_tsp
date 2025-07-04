mod graph;
mod location;
mod dijkstra;
mod tsp;

use csv::Reader;

use crate::{tsp::tsp_annealing};

fn main()   {
    let rdr = Reader::from_path("input.txt")
        .expect("Failed to open file");

    let graph = graph::construct_graph(rdr)
        .expect("Failed to construct graph");

    println!("Graph constructed with {} nodes", graph.len());


    let distance_path = tsp_annealing(&graph);

    println!("{}km", distance_path.0);

    for loc in distance_path.1{
        println!("{}", loc);
    }
}
