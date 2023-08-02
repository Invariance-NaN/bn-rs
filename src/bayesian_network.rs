use polars::export::ahash::HashMap;

use crate::graph::Digraph;

enum PDF {}

struct BayseianNetwork {
    graph: Digraph,
    pdfs: HashMap<String, PDF>
}