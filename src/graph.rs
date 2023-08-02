use std::collections::{HashSet, HashMap};


#[derive(Clone)]
pub struct Digraph {
    nodes: HashSet<String>,
    successors: HashMap<String, HashSet<String>>,
    predecessors: HashMap<String, HashSet<String>> 
}

impl Digraph {
    pub fn unconnected(nodes: impl IntoIterator<Item = String>) -> Self {
        let nodes: HashSet<String> = nodes.into_iter().collect::<HashSet<_>>();

        let mut empty_mapping = HashMap::default();

        for x in &nodes {
            empty_mapping.insert(x.clone(), HashSet::new());
        }

        Digraph {
            nodes,
            successors: empty_mapping.clone(),
            predecessors: empty_mapping
        }
    }

    pub fn fully_connected(nodes: impl IntoIterator<Item = String>) -> Self {
        let nodes: HashSet<String> = nodes.into_iter().collect::<HashSet<_>>();

        let mut full_mapping = HashMap::default();

        for x in &nodes {
            let mut neighbors = nodes.clone();
            neighbors.remove(x);
            full_mapping.insert(x.clone(), neighbors);
        }

        Digraph {
            nodes,
            successors: full_mapping.clone(),
            predecessors: full_mapping
        }
    }

    pub fn len(&self) -> usize { self.nodes.len() }

    pub fn has_node(&self, node: String) -> bool {
        self.nodes.contains(&node)
    }

    pub fn add_node(&mut self, node: String) -> bool {
        if self.nodes.insert(node.clone()) {
            self.successors.insert(node.clone(), HashSet::new());
            self.predecessors.insert(node, HashSet::new());
            true
        } else {
            false
        }
    }

    pub fn remove_node(&mut self, node: String) -> bool {
        if self.nodes.remove(&node) {

            for successor in self.successors.get(&node).unwrap() {
                self.predecessors.get_mut(successor).unwrap().remove(&node);
            }

            for predecessor in self.predecessors.get(&node).unwrap() {
                self.successors.get_mut(predecessor).unwrap().remove(&node);
            }

            self.successors.remove(&node);
            self.predecessors.remove(&node);

            true
        } else {
            false
        }  
    }

    pub fn has_edge(&self, from: String, to: String) -> Option<bool> {
        self.nodes.get(&to)?;
        self.successors.get(&from).map(|x| x.contains(&to))
    }

    pub fn add_edge(&mut self, from: String, to: String) -> Option<bool> {
        let from_successors = self.successors.get_mut(&from)?;
        let to_predecessors = self.predecessors.get_mut(&to)?;

        from_successors.insert(to);
        Some(to_predecessors.insert(from))
    }

    pub fn remove_edge(&mut self, from: String, to: String) -> Option<bool> {
        let from_successors = self.successors.get_mut(&from)?;
        let to_predecessors = self.predecessors.get_mut(&to)?;

        from_successors.remove(&to);
        Some(to_predecessors.remove(&from))
    }
}

impl Digraph {
    pub fn topological_sort(&self) -> Option<Vec<String>> {
        let mut graph = self.clone();

        let mut result = Vec::with_capacity(graph.nodes.len());

        let mut orphans = graph.predecessors.iter()
            .filter(|e| e.1.is_empty())
            .map(|e| e.0.to_owned())
            .collect::<HashSet<_>>();

        while let Some(x) = orphans.iter().next() {
            let x = x.to_owned();

            orphans.remove(&x);
            result.push(x.clone());

            let successors = graph.successors.get(&x).unwrap();
            
            for y in successors {
                let predecessors = graph.predecessors.get(y).unwrap();
                if predecessors.len() == 1 {
                    orphans.insert(y.to_owned());
                }
            }

            graph.remove_node(x);
        }        

        if graph.len() == 0 {
            Some(result)
        } else {
            None
        }
    }

    pub fn is_acyclic(&self) -> bool {
        self.topological_sort().is_some()
    }
}

impl Default for Digraph {
    fn default() -> Self { Digraph::unconnected(vec![]) }
}