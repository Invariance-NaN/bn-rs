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
    topological_sort: Option<Vec<usize>>,
    pmfs: Vec<PMF>
}

impl BayseianNetwork {
    
}

