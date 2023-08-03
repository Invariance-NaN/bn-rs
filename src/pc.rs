use crate::{graph::Digraph, dataframe::DataFrame};

fn combinations<T: Copy>(n: usize, xs: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    assert!(1 <= n && n <= xs.len());
    
    let indices = (0..n).collect::<Vec<usize>>();
    let index_to_modify = n - 1;

    std::iter::once(indices.iter().map(|&index| xs[index]).collect()).chain(
        std::iter::from_fn(move || {
            while indices[index_to_modify] + 1 >= xs.len() && 

            Some(indices.iter().map(|&index| xs[index]).collect());
        })
    )
}

fn pc(data: DataFrame) {
    let mut graph = Digraph::fully_connected(data.names().clone());

    for n in 0..graph.len() {
        // for 
    }

    // let j = data.lazy().groupby(by);
}
