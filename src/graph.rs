use std::collections::{BTreeSet, HashSet};

#[derive(Clone, Debug)]

pub struct Graph {
    names: Vec<String>,
    neighbors: Vec<BTreeSet<usize>>
}

impl Graph {
    pub fn unconnected(names: Vec<String>) -> Self {
        let neighbors = vec![BTreeSet::default(); names.len()];
        Graph { names, neighbors }
    }

    pub fn fully_connected(names: Vec<String>) -> Self {
        let all_nodes = (0..names.len()).collect::<BTreeSet<_>>();

        let mut neighbors = Vec::with_capacity(names.len());

        for i in 0..names.len() {
            let mut neighbor_set = all_nodes.clone();
            neighbor_set.remove(&i);
            neighbors.push(neighbor_set);
        }

        Graph { names, neighbors }
    }

    pub fn len(&self) -> usize { self.names.len() }

    pub fn node_index(&self, name: String) -> Option<usize> {
        self.names.iter().position(|s| s == &name)
    }

    pub fn push_node(&mut self, name: String) {
        self.names.push(name);
        self.neighbors.push(BTreeSet::default());
    }

    pub fn has_edge(&self, x: usize, y: usize) -> bool {
        assert!(y < self.len());
        self.neighbors[x].contains(&y)
    }

    pub fn add_edge(&mut self, x: usize, y: usize) -> bool {
        self.neighbors[x].insert(y);
        self.neighbors[y].insert(x)
    }

    pub fn remove_edge(&mut self, x: usize, y: usize) -> bool {
        self.neighbors[x].remove(&y);
        self.neighbors[y].remove(&x)
    }

    pub fn isolate(&mut self, node: usize) {
        self.remove_edge(node, node);

        let neighbors = self.neighbors[node].iter().copied().collect::<Vec<_>>();

        for neighbor in neighbors {
            self.neighbors[neighbor].remove(&node);
        }

        self.neighbors[node].clear();
    }

    pub fn neighbors(&self, from: usize) -> &BTreeSet<usize> {
        &self.neighbors[from]
    }

    pub fn all_edges(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.neighbors.iter()
            .enumerate()
            .flat_map(|(x, y)| y.iter().map(move |&child| (x, child)))
            .filter(|(x, y)| x <= y)
    }

    pub fn component(&self, node: usize) -> HashSet<usize> {
        assert!(node < self.len());

        let mut result = HashSet::new();
        result.insert(node);

        let mut to_check = vec![node];

        while let Some(x) = to_check.pop() {
            for &y in self.neighbors(x) {
                if result.insert(y) {
                    to_check.push(y);
                }
            }
        }

        result
    }

    pub fn min_degree(&self) -> usize {
        self.neighbors.iter().map(|x| x.len()).min().unwrap()
    }

    pub fn max_degree(&self) -> usize {
        self.neighbors.iter().map(|x| x.len()).max().unwrap()
    }

    pub fn digraph(self) -> Digraph {
        Digraph { names: self.names, children: self.neighbors.clone(), parents: self.neighbors }
    }
}


#[derive(Clone)]
pub struct Digraph {
    names: Vec<String>,
    children: Vec<BTreeSet<usize>>,
    parents: Vec<BTreeSet<usize>> 
}

// impl Digraph {
//     pub fn unconnected(names: Vec<String>) -> Self {
//         let mut empty_mapping = vec![BTreeSet::default(); names.len()];

//         Digraph {
//             names,
//             children: empty_mapping.clone(),
//             parents: empty_mapping
//         }
//     }

//     pub fn fully_connected(names: Vec<String>) -> Self {
//         let all_nodes = (0..names.len()).collect::<BTreeSet<_>>();

//         let mut full_mapping = Vec::with_capacity(names.len());

//         for i in 0..names.len() {
//             let mut neighbors = all_nodes.clone();
//             neighbors.remove(&i);
//             full_mapping.push(neighbors);
//         }

//         Digraph {
//             names,
//             children: full_mapping.clone(),
//             parents: full_mapping
//         }
//     }

//     pub fn len(&self) -> usize { self.names.len() }

//     pub fn node_index(&self, name: String) -> Option<usize> {
//         self.names.iter().position(|s| s == &name)
//     }

//     pub fn push_node(&mut self, name: String) {
//         self.names.push(name);
//         self.parents.push(BTreeSet::default());
//         self.children.push(BTreeSet::default());
//     }

//     pub fn has_edge(&self, from: usize, to: usize) -> bool {
//         assert!(to < self.len());
//         self.children[from].contains(&to)
//     }

//     pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
//         self.children[from].insert(to);
//         self.parents[to].insert(from)
//     }

//     pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
//         self.children[from].remove(&to);
//         self.parents[to].remove(&from)
//     }

//     pub fn isolate(&mut self, node: usize) {
//         self.remove_edge(node, node);

//         for &child in &self.children[node] {
//             self.parents[child].remove(&node);
//         }

//         for &parent in &self.parents[node] {
//             self.children[parent].remove(&node);
//         }

//         self.parents[node].clear();
//         self.children[node].clear();
//     }

//     pub fn children(&self, from: usize) -> &BTreeSet<usize> {
//         &self.children[from]
//     }

//     pub fn parents(&self, from: usize) -> &BTreeSet<usize> {
//         &self.children[from]
//     }

//     pub fn all_edges(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
//         self.children.iter().enumerate().flat_map(|(parent, children)| children.iter().map(move |&child| (parent, child)))
//     }

//     pub fn reachable_from(&self, node: usize) -> HashSet<usize> {
//         assert!(node < self.len());

//         let mut result = HashSet::new();
//         result.insert(node);

//         let mut to_check = vec![node];

//         while let Some(x) = to_check.pop() {
//             for &y in self.children(x) {
//                 if result.insert(y) {
//                     to_check.push(y);
//                 }
//             }
//         }

//         result
//     }
// }

// impl Digraph {
//     pub fn topological_sort(&self) -> Option<Vec<usize>> {
//         let mut graph = self.clone();

//         let mut result = Vec::with_capacity(graph.len());

//         let mut orphans = graph.parents.iter()
//             .enumerate()
//             .filter(|(_, parents)| parents.is_empty())
//             .map(|(node, _)| node)
//             .collect::<HashSet<_>>();

//         while let Some(&x) = orphans.iter().next() {
//             orphans.remove(&x);
//             result.push(x);

            
//             for &y in &graph.children[x] {
//                 let parents = graph.parents(y);

//                 if parents.len() == 1 {
//                     orphans.insert(y.to_owned());
//                 }
//             }

//             graph.isolate(x);
//         }        

//         if graph.all_edges().count() > 0 {
//             None
//         } else {
//             Some(result)
//         }
//     }

//     pub fn is_acyclic(&self) -> bool {
//         self.topological_sort().is_some()
//     }
// }

// impl Default for Digraph {
//     fn default() -> Self { Digraph::unconnected(vec![]) }
// }