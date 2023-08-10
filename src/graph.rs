use std::collections::{BTreeSet, HashSet};

use rand::random;

#[derive(Clone, Debug, PartialEq, Eq)]

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

    pub fn directed(self) -> Digraph {
        Digraph { names: self.names, children: self.neighbors.clone(), parents: self.neighbors }
    }

    pub fn connected(&self, x: usize, y: usize) -> bool {
        if x == y { return true; }

        let mut layer = vec![x];
        let mut seen: BTreeSet<_> = [x].into();

        while !layer.is_empty() {
            let mut next_layer = Vec::new();

            for &s in &layer {
                for &t in self.neighbors(s) {
                    if t == y { return true; }

                    if seen.contains(&t) { continue; }
                    next_layer.push(t);
                    seen.insert(t);
                }
            }

            layer = next_layer;
        }

        false
    }

    pub fn degree(&self, node: usize) -> usize {
        self.neighbors(node).len()
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Digraph {
    names: Vec<String>,
    children: Vec<BTreeSet<usize>>,
    parents: Vec<BTreeSet<usize>>
}

impl Digraph {
    pub fn unconnected(names: Vec<String>) -> Self {
        Graph::unconnected(names).directed()
    }

    pub fn fully_connected(names: Vec<String>) -> Self {
        Graph::fully_connected(names).directed()
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
        self.parents[to].insert(from)
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        self.children[from].remove(&to);
        self.parents[to].remove(&from)
    }

    pub fn isolate(&mut self, node: usize) {
        self.remove_edge(node, node);

        for &child in &self.children[node] {
            self.parents[child].remove(&node);
        }

        for &parent in &self.parents[node] {
            self.children[parent].remove(&node);
        }

        self.parents[node].clear();
        self.children[node].clear();
    }

    pub fn children(&self, from: usize) -> &BTreeSet<usize> {
        &self.children[from]
    }

    pub fn parents(&self, from: usize) -> &BTreeSet<usize> {
        &self.parents[from]
    }

    pub fn all_edges(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.children.iter().enumerate()
            .flat_map(|(parent, children)| children.iter().map(
                move |&child| (parent, child)
            ))
    }

    pub fn ancestors(&self, nodes: Vec<usize>) -> Vec<usize> {
        let mut result: BTreeSet<usize> = nodes.iter().copied().collect();
        let mut to_check = nodes;

        while let Some(x) = to_check.pop() {
            for &y in self.parents(x) {
                if result.contains(&y) { continue; }
                result.insert(y);
                to_check.push(y);
            }
        }

        result.into_iter().collect()
    }

    pub fn subgraph(self, names: Vec<usize>) -> Self {
        let names: BTreeSet<usize> = names.into_iter().collect();

        Digraph {
            names: self.names,
            children: self.children.into_iter()
                .enumerate()
                .filter(|(x, _)| !names.contains(x))
                .map(|(_, ys)| ys.into_iter().filter(|y| !names.contains(y)).collect())
                .collect(),
            parents: self.parents.into_iter()
                .enumerate()
                .filter(|(x, _)| !names.contains(x))
                .map(|(_, ys)| ys.into_iter().filter(|y| !names.contains(y)).collect())
                .collect()
        }
    }

    pub fn undirected(self) -> Graph {
        Graph {
            names: self.names,
            neighbors: self.parents.iter().zip(self.children.iter())
                .map(|(xs, ys)| xs.union(ys).copied().collect())
                .collect()
        }
    }
}

impl Digraph {
    pub fn erdos_renyi(size: usize, p: f64) -> Self {
        let names = (0..size).map(|x| x.to_string()).collect();
        let mut result = Digraph::unconnected(names);

        for i in 0..size {
            for j in (1 + i)..size {
                if random::<f64>() < p {
                    result.add_edge(i, j);
                }
            }
        }

        result
    }

    pub fn moralize(mut self) -> Self {
        let mut to_check: BTreeSet<usize> = (0..self.len()).collect();

        while let Some(&z) = to_check.first() {
            to_check.remove(&z);

            let parents = self.parents(z).iter().copied().collect::<Vec<_>>();

            for i in 0..parents.len() {
                let x = parents[i];
                for j in (i + 1)..parents.len() {
                    let y = parents[j];

                    if !(self.has_edge(x, y) || self.has_edge(y, x)) {
                        self.add_edge(x, y);
                    }
                }
            }
        }

        self
    }

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

            graph.isolate(x);
        }

        if graph.all_edges().count() > 0 {
            None
        } else {
            Some(result)
        }
    }

    pub fn d_separated(&self, x: usize, y: usize, zs: Vec<usize>) -> bool {
        let ancestors: BTreeSet<_> = {
            let mut nodes = zs.clone();
            nodes.push(x);
            nodes.push(y);
            self.clone().ancestors(nodes).into_iter().collect()
        };

        let mut graph = self.clone();

        for node in 0..graph.len() {
            if ancestors.contains(&node) { continue; }
            graph.isolate(node);
        }


        let mut graph = graph.moralize().undirected();

        for &z in &zs {
            graph.isolate(z);
        }

        let result = !graph.connected(x, y);
        result
    }

    pub fn is_acyclic(&self) -> bool {
        self.topological_sort().is_some()
    }
}

impl Default for Digraph {
    fn default() -> Self { Digraph::unconnected(vec![]) }
}
