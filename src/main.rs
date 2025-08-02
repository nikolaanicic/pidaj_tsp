mod graph;
mod location;
mod tsp;

use std::{fs::File, io::{Write}, sync::{mpsc, Arc}, thread};

use csv::Reader;

use crate::{location::Location, tsp::tsp_annealing};

const NTHREADS:i32 = 16;

fn main()   {
    let rdr = Reader::from_path("input.txt")
        .expect("Failed to open file");

    let graph = graph::construct_graph(rdr)
        .expect("Failed to construct graph");
    let graph = Arc::new(graph);


    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    for _ in 0..=NTHREADS{
        let tx = tx.clone();
        let gph = Arc::clone(&graph);
        let handle = thread::spawn(move ||{
            let result = tsp_annealing(gph);
            tx.send(result).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    drop(tx);

    let mut best_distance = std::i32::MAX;
    let mut best_route:Vec<Arc<Location>> = vec![];

    while let Ok(received) = rx.recv() {
        if received.0 < best_distance{
            best_distance = received.0;
            best_route = received.1;
        }
    }

    println!("{}", best_distance);

    for loc in best_route.iter(){
        println!("{}", *loc);
    }

    let mut output = File::create("output.txt").unwrap();

    writeln!(output, "{}", best_distance).unwrap();

    for loc in best_route.iter(){
        writeln!(output, "{},{}", loc.city,loc.country).unwrap();
    }

}
