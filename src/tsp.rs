use std::{sync::Arc};
use rand::prelude::*;
use crate::{location::Location, graph::Graph};


fn get_random_route(g: &Graph) -> Vec<Arc<Location>>{
    let mut rng = rand::rng();
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

fn is_swap_valid(route: &Vec<Arc<Location>>, g: &Graph, i: usize, j: usize) -> bool {
    let len = route.len();

    // Helper closures for cyclic indexing (exclude last city because it's duplicate)
    let prev = |x: usize| if x == 0 { len - 2 } else { x - 1 };
    let next = |x: usize| if x == len - 2 { 0 } else { x + 1 };

    // Cities after swap
    let city_i = &route[j]; // city j moved to position i
    let city_j = &route[i]; // city i moved to position j

    // Check edges around position i
    if !g.get(&route[prev(i)])
         .and_then(|nbrs| nbrs.get(city_i))
         .is_some() {
        return false;
    }
    if !g.get(city_i)
         .and_then(|nbrs| nbrs.get(&route[next(i)]))
         .is_some() {
        return false;
    }

    // Check edges around position j
    if !g.get(&route[prev(j)])
         .and_then(|nbrs| nbrs.get(city_j))
         .is_some() {
        return false;
    }
    if !g.get(city_j)
         .and_then(|nbrs| nbrs.get(&route[next(j)]))
         .is_some() {
        return false;
    }

    true
}

fn get_route_perturbation(route: &Vec<Arc<Location>>, g: &Graph) -> Vec<Arc<Location>> {
    let mut rng = rand::rng();
    let len = route.len();
    const MAX_TRIES: usize = 100;

    for _ in 0..MAX_TRIES {
        let i = rng.random_range(0..len - 1);
        let mut j = rng.random_range(0..len - 1);
        while j == i {
            j = rng.random_range(0..len - 1);
        }

        if is_swap_valid(route, g, i, j) {
            let mut new_route = route.clone();
            new_route.swap(i, j);

            new_route[len - 1] = new_route[0].clone();

            return new_route;
        }
    }

    for _ in 0..MAX_TRIES {
        let i = rng.random_range(0..len - 2);
        let j = rng.random_range(i + 1..len - 1);
        let mut new_route = route[0..len - 1].to_vec();
        new_route[i..=j].reverse();
        new_route.push(new_route[0].clone());
        if is_swap_valid(&new_route, g, i, j) {
            return new_route;
        }
    }

    // If no valid swap found, return original route (could do more sophisticated fallback)
    route.clone()
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
        let next_route = get_route_perturbation(&current_route, g);
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