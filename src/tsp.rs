use std::{sync::Arc};
use rand::prelude::*;
use crate::{location::Location, graph::Graph};


fn get_random_route(g: &Graph) -> Vec<Arc<Location>>{
    let mut rng = rand::rng();

    // Start with a shuffled list of cities
    let mut route: Vec<Arc<Location>> = g.keys().cloned().collect();
    route.shuffle(&mut rng);

    while !g.get(&route[route.len() - 1])
             .and_then(|neighbors| neighbors.get(&route[0]))
             .is_some() {
        route.shuffle(&mut rng);
    }

    route.push(route[0].clone());
    route
}

fn get_route_perturbation(route: &Vec<Arc<Location>>) -> Vec<Arc<Location>>{
    let mut rng = rand::rng();
    let indices: Vec<_> = (0..route.len() - 1).collect();
    
    let sampled: Vec<_> = indices.iter().choose_multiple(&mut rng, 2);
    let (i, j) = (*sampled[0], *sampled[1]);

    let mut new_route = route[0..route.len() - 1].to_vec();
    new_route.swap(i, j);
    new_route.push(new_route[0].clone());

    new_route
}


fn distance(route: &Vec<Arc<Location>>, g: &Graph) -> i32{
    let mut distance = 0.0 as i32;

    for i in  0..route.len()-1{
		let start = &route[i];
		let end = &route[i+1];
        distance += g.get(start)
			.and_then(|neighbors| neighbors.get(end))
			.copied()
			.unwrap_or(0);
    }

    distance
}

pub fn tsp_annealing(g: Arc<Graph>) -> (i32, Vec<Arc<Location>>){
    let g = &*g;
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