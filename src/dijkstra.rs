use std::{collections::{BinaryHeap, HashMap, HashSet}, rc::Rc};
use crate::{location::Location, graph::Graph};



pub type DijkstraTable = HashMap<Rc<Location>, DijkstraTableRow>;

pub struct DijkstraTableRow{
	pub cost:f32,
	pub path: Vec<Rc<Location>>
}

struct DijkstraHeapState{
	cost:f32,
	location:Rc<Location>
}


impl Ord for DijkstraHeapState{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.cost.total_cmp(&other.cost)
	}
}

impl PartialOrd for DijkstraHeapState{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for DijkstraHeapState{
	fn eq(&self, other: &Self) -> bool {
		self.cost == other.cost && self.location == other.location
	}
}

impl Eq for DijkstraHeapState {}



pub fn dijkstra(g: &Graph, start: Rc<Location>) -> DijkstraTable{
    let mut distances: DijkstraTable = DijkstraTable::new();
    
    // Initialize all distances to INFINITY
    for (loc, _) in g.into_iter() {
        distances.insert(loc.clone(), DijkstraTableRow {
            cost: f32::INFINITY,
            path: Vec::new(),
        });
    }

    distances.get_mut(&start).unwrap().cost = 0.0;
    distances.get_mut(&start).unwrap().path.push(start.clone());

    let mut heap = BinaryHeap::new();
    heap.push(DijkstraHeapState {
        cost: 0.0,
        location: start.clone(),
    });

    let mut visited = HashSet::new();

	while let Some(DijkstraHeapState{cost,location}) = heap.pop(){
        if visited.contains(&location) {
            continue;
        }
        visited.insert(location.clone());


        let current_cost = distances.get(&location).unwrap().cost;
        if cost > current_cost {
            continue;
        }

        let neighbors = match g.get(&location) {
            Some(n) => n,
            None => continue,
        };

        for (neighbor_loc, neighbor_cost) in neighbors {
            let new_cost = cost + *neighbor_cost as f32;
            
            let should_update = {
                let neighbor_distance = distances.get(neighbor_loc).unwrap();
                new_cost < neighbor_distance.cost
            };
            
            if should_update {
                let current_path = distances.get(&location).unwrap().path.clone();
                
                let entry = distances.get_mut(neighbor_loc).unwrap();
                entry.cost = new_cost;
                entry.path = current_path;
                entry.path.push(neighbor_loc.clone());
                
                heap.push(DijkstraHeapState {
                    cost: new_cost,
                    location: neighbor_loc.clone(),
                });
            }
        }

	}


	distances

}

