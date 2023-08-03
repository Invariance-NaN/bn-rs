use std::collections::{BTreeSet, HashSet};


#[derive(Clone)]
pub struct Digraph {
    names: Vec<String>,
    children: Vec<BTreeSet<usize>>,
    parents: Vec<BTreeSet<usize>> 
}

impl Digraph {
    pub fn unconnected(names: Vec<String>) -> Self {
        let mut empty_mapping = vec![BTreeSet::default(); names.len()];

        Digraph {
            names,
            children: empty_mapping.clone(),
            parents: empty_mapping
        }
    }

    pub fn fully_connected(names: Vec<String>) -> Self {
        let all_nodes = (0..names.len()).collect::<BTreeSet<_>>();

        let mut full_mapping = Vec::with_capacity(names.len());

        for i in 0..names.len() {
            let mut neighbors = all_nodes.clone();
            neighbors.remove(&i);
            full_mapping.push(neighbors);
        }

        Digraph {
            names,
            children: full_mapping.clone(),
            parents: full_mapping
        }
    }

    pub fn len(&self) -> usize { self.names.len() }

    pub fn node_index(&self, name: String) -> Option<usize> {
        self.names.iter().position(|s| s == &name)
    }

    pub fn push_node(&mut self, name: String) {
        self.names.push(name);
        self.parents.push(BTreeSet::default());
        self.children.push(BTreeSet::default());
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        assert!(to < self.len());
        self.children[from].contains(&to)
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        self.children[from].insert(to);
        self.children[to].insert(from)
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        self.children[from].remove(&to);
        self.children[to].remove(&from)
    }

    pub fn remove_all_edges_from(&mut self, from: usize) {
        for &to in &self.children[from] {
            self.parents[to].remove(&from);
        }

        self.children[from].clear();
    }

    pub fn children(&self, from: usize) -> &BTreeSet<usize> {
        &self.children[from]
    }

    pub fn parents(&self, from: usize) -> &BTreeSet<usize> {
        &self.children[from]
    }

    pub fn all_edges(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.children.iter().enumerate().flat_map(|(parent, children)| children.iter().map(move |&child| (parent, child)))
    }
}

impl Digraph {
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let mut graph = self.clone();

        let mut result = Vec::with_capacity(graph.len());

        let mut orphans = graph.parents.iter()
            .enumerate()
            .filter(|(_, parents)| parents.is_empty())
            .map(|(node, _)| node)
            .collect::<HashSet<_>>();

        while let Some(&x) = orphans.iter().next() {
            orphans.remove(&x);
            result.push(x);

            
            for &y in &graph.children[x] {
                let parents = graph.parents(y);

                if parents.len() == 1 {
                    orphans.insert(y.to_owned());
                }
            }

            graph.remove_all_edges_from(x);
        }        

        if graph.all_edges().count() > 0 {
            None
        } else {
            Some(result)
        }
    }

    pub fn is_acyclic(&self) -> bool {
        self.topological_sort().is_some()
    }
}

impl Default for Digraph {
    fn default() -> Self { Digraph::unconnected(vec![]) }
}