use std::collections::{HashSet, HashMap};
use rayon::prelude::*;

use crate::{graph::{Graph, Digraph}, dataframe::DataFrame, iter::{combinations, interleave}};

type SeperationSets = HashMap<(usize, usize), HashSet<usize>>;

fn dual_iter(min: usize, max: usize) -> impl Iterator<Item = usize> {
    //! Yields max, min, max - 1, min + 1, ...

    interleave((min..=max).rev(), min..=max).take(max - min + 1)
}

fn orient_edges(graph: &mut Graph, sep_sets: &SeperationSets) {

}

pub fn pc(data: DataFrame, answer: Digraph) -> (Graph, u32) {
    let mut ci_tests = 0;

    let mut graph = Graph::fully_connected(data.names().clone());
    let mut indices: HashSet<usize> = (0..graph.len()).collect();
    let mut sep_sets: SeperationSets = HashMap::new();

    for n in 0..=graph.len() - 2 {
        if n > graph.max_degree() { continue; }

        for x in 0..graph.len() {
            indices.remove(&x);

            for y in (x + 1)..graph.len() {
                if !graph.has_edge(x, y) { continue; }

                indices.remove(&y);

                for zs in combinations(n, indices.iter().collect()) {
                    let zs: Vec<_> = zs.into_iter().copied().collect();

                    ci_tests += 1;
                    if data.fake_conditionally_independent(x, y, zs.clone(), &answer) {
                        graph.remove_edge(x, y);
                        graph.remove_edge(y, x);
                        sep_sets.insert((x, y), zs.into_iter().collect());
                    }
                }

                indices.insert(y);
            }

            indices.insert(x);
        }
    }

    orient_edges(&mut graph, &sep_sets);

    return (graph, ci_tests);
}

pub fn pc_dual(data: DataFrame, answer: Digraph) -> (Graph, u32) {
    let mut ci_tests = 0;

    let mut graph = Graph::fully_connected(data.names().clone());
    let mut indices: HashSet<usize> = (0..graph.len()).collect();
    let mut sep_sets: SeperationSets = HashMap::new();

    for n in dual_iter(0, graph.len() - 2) {
        if n > graph.max_degree() { continue; }

        for x in 0..graph.len() {
            indices.remove(&x);

            for y in (x + 1)..graph.len() {
                if !graph.has_edge(x, y) { continue; }

                indices.remove(&y);

                for zs in combinations(n, indices.iter().collect()) {
                    let zs: Vec<_> = zs.into_iter().copied().collect();

                    ci_tests += 1;
                    if data.fake_conditionally_independent(x, y, zs.clone(), &answer) {
                        graph.remove_edge(x, y);
                        graph.remove_edge(y, x);
                        sep_sets.insert((x, y), zs.into_iter().collect());
                    }
                }

                indices.insert(y);
            }

            indices.insert(x);
        }
    }

    orient_edges(&mut graph, &sep_sets);

    return (graph, ci_tests);
}

fn wb(graph: &Graph, x: usize, y: usize) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
    let mut w = HashSet::new();
    let mut b = HashSet::new();

    let mut graph_prime = graph.clone();
    graph_prime.isolate(x);
    graph_prime.isolate(y);

    for z in graph.neighbors(x).intersection(graph.neighbors(y)) {
        let r = graph_prime.component(*z);
        for x in r.drain() {
            w.insert(x);
        }
    }

    let c: HashSet<usize> = {
        let result =
    };

    for &z in graph.neighbors(x) {
        if graph_prime.degree(z) > 0 {
            graph_prime.add_edge(x, z);
        }
    }

    for &z in graph.neighbors(y) {
        if graph_prime.degree(z) > 0 {
            graph_prime.add_edge(x, y);
        }
    }


    for &z in graph_prime.neighbors(x) {
        for &v in graph_prime.neighbors(z) {
            if v == x { continue; }

            let r_v = graph_prime.component(v);

            if r_v.into_iter().any(|w| graph.has_edge(w, z)) {
                b.insert(z);
                b.insert(v);
            }
        }
    }

    (w, b, c)
}


pub fn shortcut_pc_dual(data: DataFrame, answer: Digraph) -> (Graph, u32) {
    let mut ci_tests = 0;

    let mut graph = Graph::fully_connected(data.names().clone());
    let mut indices: HashSet<usize> = (0..graph.len()).collect();
    let mut sep_sets: SeperationSets = HashMap::new();

    for m in dual_iter(0, graph.len() - 2) {
        if m > graph.max_degree() { continue; }

        for x in 0..graph.len() {
            for y in graph.neighbors(x).clone() {
                let (s, b, c) = wb(&graph, x, y);

                if (x, y) == (2, 4) || (x, y) == (4, 2) {

                }

                if b.len() > m { continue; }
                for u in combinations(m - b.len(), s.iter().collect()) {
                    let u = u.into_iter().copied().collect();
                    let sep_set: Vec<_> = b.union(&u).copied().collect();

                    ci_tests += 1;
                    if data.fake_conditionally_independent(x, y, sep_set.clone(), &answer) {
                        graph.remove_edge(x, y);
                        sep_sets.insert((x, y), sep_set.into_iter().collect());
                    }
                }
            }

            indices.insert(x);
        }
    }

    orient_edges(&mut graph, &sep_sets);

    return (graph, ci_tests);
}


pub fn shortcut_pc(data: DataFrame, answer: Digraph) -> (Graph, u32) {
    let mut ci_tests = 0;

    let mut graph = Graph::fully_connected(data.names().clone());
    let mut indices: HashSet<usize> = (0..graph.len()).collect();
    let mut sep_sets: SeperationSets = HashMap::new();


    for m in 0..=graph.len() - 2 {
        if m > graph.max_degree() { continue; }
        println!("m = {}", m);
        for x in 0..graph.len() {
            for y in graph.neighbors(x).clone() {
                let (s, b, _c) = wb(&graph, x, y);

                if x == 2 && y == 4 {
                    println!();
                    println!("in 2-4_G: {:?}", graph.clone());
                    println!("in 2-4_s: {:?}", s.clone());
                    println!("in 2-4_b: {:?}", b.clone());
                }

                if b.len() > m { continue; }
                for u in combinations(m - b.len(), s.iter().collect()) {
                    let u = u.into_iter().copied().collect();
                    let sep_set: Vec<_> = b.union(&u).copied().collect();

                    ci_tests += 1;
                    if data.fake_conditionally_independent(x, y, sep_set.clone(), &answer) {
                        println!(" Removing node {} -- {}", x, y);
                        println!("  S_xy: {:?}", s.iter().copied().collect::<Vec<_>>());
                        println!("  B_xy: {:?}", b.iter().copied().collect::<Vec<_>>());
                        println!("  sepset: {:?}", sep_set);
                        graph.remove_edge(x, y);
                        sep_sets.insert((x, y), sep_set.into_iter().collect());
                    }
                }
            }

            indices.insert(x);
        }
    }

    orient_edges(&mut graph, &sep_sets);

    return (graph, ci_tests);
}



mod tests {
    use super::*;
    use std::time::{Instant, Duration};
    use rand::prelude::*;
    use crate::{dataframe::DataFrame, graph::{self, Digraph}};

    #[test]
    fn sanity() {
        let nodes: Vec<_> = (0..6).map(|x| x.to_string()).collect();

        // let graph = crate::fixed_graphs::munin();

        let graph = {
            let mut graph = Digraph::unconnected(nodes.clone());
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(0, 4);
            graph.add_edge(4, 5);
            graph
        };
        // 0 -> 1 -> 2 -> 3
        // + -> 4 -> 5

        let df = {
            let mut df = DataFrame::new(nodes.clone());
            df.add_row(nodes.iter().map(|_| 0).collect());
            df
        };

        fn test(
            f: impl Fn(DataFrame, Digraph) -> (Graph, u32),
            df: &DataFrame,
            graph: &Digraph
        ) -> (Graph, u32, Duration) {
            let df = df.clone();
            let graph = graph.clone();
            let start = Instant::now();
            let (result, ci) = f(df, graph);
            let time = start.elapsed();
            (result, ci, time)
        }

        let (result_pc, ci_pc, time_pc) = test(pc, &df, &graph);
        let (result_sc, ci_sc, time_sc) = test(shortcut_pc, &df, &graph);
        let (result_pd, ci_pd, time_pd) = test(pc_dual, &df, &graph);
        let (result_sd, ci_sd, time_sd) = test(shortcut_pc_dual, &df, &graph);

        let ud = graph.clone().undirected();
        println!("===========");
        println!("expected: {a:?}\nexpected-undirected: {b:?}", b=graph.clone().undirected(),a=graph.clone());
        println!("pc: {:?}", result_pc.clone());
        println!("sc: {:?}", result_sc.clone());
        println!("pc: {:?} CI, {} ms (off by {})", ci_pc, time_pc.as_millis(), result_pc.edge_difference(&ud));
        println!("sc: {:?} CI, {} ms (off by {})", ci_sc, time_sc.as_millis(), result_sc.edge_difference(&ud));
        println!("pd: {:?} CI, {} ms (off by {})", ci_pd, time_pd.as_millis(), result_pd.edge_difference(&ud));
        println!("sd: {:?} CI, {} ms (off by {})", ci_sd, time_sd.as_millis(), result_sd.edge_difference(&ud));

        // assert_eq!(result_pc, graph.clone().undirected());
        // assert_eq!(result_pc, result_sc);
        // assert_eq!(result_pc, result_pd);
        // assert_eq!(result_pc, result_sd);
    }
}
