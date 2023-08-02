use crate::graph::Digraph;

use polars::prelude::*;



fn conditional_independency(data: DataFrame, x: String, y: String, z: Vec<String>) {
    data.lazy()
        .groupby(z.iter().map(|x| col(x)).collect::<Vec<_>>())
        .agg([col(&x).min(), col(&y).min()]);

}

fn pc(data: DataFrame) {
    let mut graph = Digraph::fully_connected(
        data.get_column_names().into_iter().map(|x| x.to_owned()).collect::<Vec<_>>()
    );

   

    // let j = data.lazy().groupby(by);
}
