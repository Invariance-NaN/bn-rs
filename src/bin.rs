pub mod bayesian_network;
pub mod iter;
pub mod dataframe;
pub mod graph;
pub mod structure_learning;

use std::time::Instant;

use crate::structure_learning::pc::*;
use crate::dataframe::*;
use crate::graph::*;
use rand::prelude::*;

use rayon::iter::repeatn;
use rayon::prelude::*;

fn main() {
    println!("[");
    for n in [10,12,14,16] {
        for p in (0..=10).map(|x| x as f64 / 10.0) {
            repeatn(0, 100).for_each(|_| {
                let graph = Digraph::erdos_renyi(n, p);

                let data_1 = {
                    let mut result = DataFrame::new(
                        (0..graph.len()).map(|x| x.to_string()).collect()
                    );

                    let row = result.names().iter().map(|_| random::<u32>()).collect::<Vec<_>>();

                    for _ in 0..1000 {
                        result.add_row(row.clone());
                    }

                    result
                };

                let data_2 = data_1.clone();
                let data_3 = data_1.clone();
                let data_4 = data_1.clone();

                let graph_1 = graph.clone();
                let graph_2 = graph.clone();
                let graph_3 = graph.clone();
                let graph_4 = graph.clone();

                let start_pc = Instant::now();
                let (result_pc, ci_pc) = pc(data_1, graph_1);
                let pc_time = start_pc.elapsed();

                let start_sc = Instant::now();
                let (result_sc, ci_sc) = shortcut_pc(data_2, graph_2);
                let sc_time = start_sc.elapsed();
                
                let start_pd = Instant::now();
                let (result_pd, ci_pd) = pc_dual(data_3, graph_3);
                let pd_time = start_pd.elapsed();

                let start_sd = Instant::now();
                let (result_sd, ci_sd) = shortcut_pc_dual(data_4, graph_4);
                let sd_time = start_sd.elapsed();

                {
                    use std::io::Write;

                    let mut stdout = std::io::stdout().lock();
                    writeln!(&mut stdout, "{{").unwrap();
                    writeln!(&mut stdout, "n: {},", n).unwrap();
                    writeln!(&mut stdout, "p: {},", p).unwrap();
                    writeln!(&mut stdout, "pc: {{ms: {}, ci: {}}},", pc_time.as_millis(), ci_pc).unwrap();
                    writeln!(&mut stdout, "sc: {{ms: {}, ci: {}}},", sc_time.as_millis(), ci_sc).unwrap();
                    writeln!(&mut stdout, "pd: {{ms: {}, ci: {}}},", pd_time.as_millis(), ci_pd).unwrap();
                    writeln!(&mut stdout, "sd: {{ms: {}, ci: {}}}", sd_time.as_millis(), ci_sd).unwrap();
                    writeln!(&mut stdout, "}}").unwrap();
                }

                if result_pc != graph.clone().undirected() || result_pc != result_sc || result_pc != result_pd || result_pc != result_sd {
                    println!("## NEQ ##");
                    println!("#actual: {a:?}\nactual-undirected: {b:?}", b=graph.clone().undirected(),a=graph);
                    println!("#pc: {:?}", result_pc);
                    println!("#sc: {:?}", result_sc);
                    println!("#pd: {:?}", result_pd);
                    println!("#sd: {:?}", result_sd);
                }
            });
        }
    }
    println!("]");
}
