use std::collections::{HashMap, HashSet};
use std::{env, io::BufRead};

type Graph = HashMap<usize, HashMap<usize, usize>>;

fn dijkstra(graph: &Graph, source: usize) -> HashMap<usize, usize> {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();

    distances.insert(source, 0);

    for _ in 0..graph.len() {
        let mut min_distance = usize::MAX;
        let mut current_node = 0;
        for (node, &distance) in &distances {
            if !visited.contains(node) && distance < min_distance {
                min_distance = distance;
                current_node = *node;
            }
        }

        visited.insert(current_node);

        if let Some(neighbors) = graph.get(&current_node) {
            for (neighbor, &weight) in neighbors {
                let new_distance = min_distance + weight;
                if !visited.contains(neighbor) {
                    distances
                        .entry(*neighbor)
                        .and_modify(|d| *d = (*d).min(new_distance))
                        .or_insert(new_distance);
                }
            }
        }
    }

    distances
}

fn main() -> std::io::Result<()> {
    let mut graph: Graph = HashMap::new();
    let args = env::args().collect::<Vec<String>>();
    let fs = std::fs::OpenOptions::new()
        .read(true)
        .open(args.get(1).map_or("./data.txt", |arg| &*arg))?;
    let r = std::io::BufReader::new(fs);
    r.lines().enumerate().for_each(|(idx, res)| match res {
        Ok(value) => {
            let c = value
                .split_whitespace()
                .filter_map(|x| match x.parse::<usize>() {
                    Ok(value) => Some(value),
                    Err(_) => None,
                })
                .collect::<Vec<usize>>();

            c.iter().enumerate().for_each(|(key, &value)| {
                if value != 0 {
                    if !graph.contains_key(&idx) {
                        graph.insert(idx, HashMap::from([(key, value)]));
                    } else {
                        if let Some(i) = graph.get_mut(&idx) {
                            i.insert(key, value);
                        }
                    }
                }
            });
        }
        Err(e) => println!("ERROR {:?}", e),
    });

    let distances = dijkstra(&graph, 0);

    // Print the distances from the source to all nodes
    for (node, distance) in distances {
        println!("NODE: {} -> DISTANCE: {}", node, distance);
    }

    Ok(())
}
