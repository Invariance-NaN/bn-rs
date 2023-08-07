use std::collections::HashMap;
use rand::prelude::*;

use crate::graph::Digraph;

struct PMF {
    table: HashMap<Vec<u32>, Vec<(u32, f64)>>
}

impl PMF {
    pub fn sample(&self, parents: Vec<u32>) -> u32 {
        let mut rand: f64 = rand::random();

        for &(outcome, probability) in &self.table[&parents] {
            if rand < probability {
                return outcome;
            }

            rand -= probability;
        }

        // A very unlikely edge case caused by floating-point rounding issues.
        *parents.last().unwrap()
    }
}

struct BayseianNetwork {
    graph: Digraph,
    topological_sort: Vec<usize>,
    pmfs: Vec<PMF>
}

impl BayseianNetwork {
    pub fn new(graph: Digraph, pmfs: Vec<PMF>) -> BayseianNetwork {
        assert!(graph.len() == pmfs.len());

        let topological_sort = graph.topological_sort().unwrap();

        BayseianNetwork { graph, topological_sort, pmfs }
    }

    pub fn len(&self) -> usize {
        self.topological_sort.len()
    }

    pub fn sample(&self) -> Vec<u32> {
        let mut result = vec![0; self.len()];

        for &x in &self.topological_sort {
            let parents = self.graph.parents(x);

            let mut parent_values = Vec::with_capacity(parents.len());

            for &y in parents {
                parent_values.push(result[y]);
            }

            result[x] = self.pmfs[x].sample(parent_values);
        }

        result
    }
}
