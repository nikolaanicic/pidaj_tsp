use std::{rc::Rc};
use rand::prelude::*;
use crate::{location::Location, graph::Graph};


fn get_random_route(g: &Graph) -> Vec<Rc<Location>>{
    let mut route: Vec<Rc<Location>> = g.keys().cloned().collect();
	let mut rng = rand::rng();

	route.shuffle(&mut rng);

    route
}

fn get_route_perturbation(route: &Vec<Rc<Location>>) -> Vec<Rc<Location>>{
	let mut rng = rand::rng();

    
    // Generate indices from 0 to route.len()-2 (equivalent to Python's range(len(route)-1))
    let indices: Vec<_> = (0..route.len()-1).collect();
    
    // Sample 2 distinct indices
    let sampled: Vec<_> = indices.iter().choose_multiple(&mut rng, 2);
    let (i, j) = (*sampled[0], *sampled[1]);

	let mut new_route = route.clone();
	new_route.swap(i, j);

	new_route
}


fn distance(route: &Vec<Rc<Location>>, g: &Graph) -> i32{
    let mut distance = 0.0 as i32;

    for i in  0..route.len()-1{
		let start = &route[i];  // Borrow instead of clone
		let end = &route[i+1];
        distance += g.get(start)
			.and_then(|neighbors| neighbors.get(end))
			.copied()
			.unwrap_or(0);
    }

    distance
}

pub fn tsp_annealing(g: &Graph) -> (i32, Vec<Rc<Location>>){
    let mut rng = rand::rng();  
    let mut temperature = 10000.0;
    let cooling_rate = 0.95;
    let min_temperature = 1e-3;
    const MAX_ITERATIONS: usize = 100000;

    let mut current_route = get_random_route(g);  
    let mut current_distance = distance(&current_route, g);

    let mut best_route = current_route.clone(); 
    let mut best_distance = current_distance;
    let mut iteration = 0;

    while temperature > min_temperature && iteration < MAX_ITERATIONS {
        let next_route = get_route_perturbation(&current_route);
        let next_distance = distance(&next_route, g);

        let delta = current_distance as f32 - next_distance as f32;
        let acceptance_prob = (delta / temperature).exp();

        if next_distance < current_distance || rng.random::<f32>() < acceptance_prob {
            current_route = next_route;  
            current_distance = next_distance;

            if current_distance < best_distance {
                best_distance = current_distance;
                best_route = current_route.clone(); 
            }
        }

        temperature *= cooling_rate;
        iteration += 1;
    }

    (best_distance, best_route)
}