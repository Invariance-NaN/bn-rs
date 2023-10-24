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
        println!("pc: n = {}", n);

        for x in 0..graph.len() {
            indices.remove(&x);

            for y in (x + 1)..graph.len() {
                if !graph.has_edge(x, y) { continue; }
                println!("  x = {}, y = {}", x, y);

                indices.remove(&y);

                for zs in combinations(n, &indices.iter().collect::<Vec<_>>()) {
                    println!("    zs = {:?}", zs);
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

                for zs in combinations(n, &indices.iter().collect::<Vec<_>>()) {
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

fn wbc(graph: &Graph, x: usize, y: usize) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
    let mut w = HashSet::new();
    let mut b = HashSet::new();

    // let (mut s, mut b, mut c) = {
    //     let mut c = (0..graph.len()).collect::<HashSet<_>>();
    //     c.remove(&x);
    //     c.remove(&y);
    //     (HashSet::new(), HashSet::new(), c)
    // };

    let mut graph_prime = graph.clone();
    graph_prime.isolate(x);
    graph_prime.isolate(y);

    for z in graph.neighbors(x).intersection(graph.neighbors(y)) {
        let mut r = graph_prime.component(*z);
        for node in r.drain() {
            w.insert(node);
        }
    }

    let mut c = (0..graph.len()).collect::<HashSet<_>>();
    c.remove(&x);
    c.remove(&y);
    for node in w.iter() {  c.remove(node); }

    for &node in w.iter() {
        graph_prime.isolate(node);
    }

    let mut graph_2 = graph.clone();
    for &node in w.iter() {
        if node != y { graph_2.isolate(node); }
    }
    for &node in graph.neighbors(x) {
        if node != y { graph_2.isolate(node); }
    }

    let r_prime = graph_2.component(y);

    for &z in graph.neighbors(x) {
        if c.contains(&z) && graph_prime.neighbors(z).iter()
            .any(|node| r_prime.contains(node))
        {
            b.insert(z);
            for &node in graph_prime.neighbors(z) {
                b.insert(node);
            }
        }
    }

    // for &z in graph_prime.neighbors(x) {
    //     for &v in graph_prime.neighbors(z) {
    //         if v == x { continue; }

    //         let r_v = graph_prime.component(v);

    //         if r_v.into_iter().any(|w| graph.has_edge(w, z)) {
    //             b.insert(z);
    //             b.insert(v);
    //         }
    //     }
    // }

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
                let (s, b, c) = wbc(&graph, x, y);

                if b.len() > m { continue; }
                for u in combinations(m - b.len(), &s.iter().collect::<Vec<_>>()) {
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


    for m in 0..=graph.len() {
        if m > graph.max_degree() { break; }

        for x in 0..graph.len() {
            for y in graph.neighbors(x).clone() {
                let (w, b, _c) = wbc(&graph, x, y);

                let k = {
                    let mut result: usize = 0;
                    for node in graph.neighbors(x) {
                        if b.contains(node) {
                            result += 1;
                        }
                    }
                    result
                };

                let u_super: Vec<usize> = {
                    let mut result = Vec::new();

                    for node in graph.neighbors(x) {
                        if w.contains(node) {
                            result.push(*node);
                        }
                    }

                    result
                };
                println!("(x, y, usuper): {:?}", (x, y, &u_super));
                for size in m.saturating_sub(k) ..= m {
                    println!("  size: {:?}", size);
                    for u in combinations(size, &u_super) {
                        let u = u.into_iter().collect();
                        let sep_set: Vec<_> = b.union(&u).copied().collect();

                        println!("SC Test: {} indep {} | {:?}", x, y, sep_set);

                        ci_tests += 1;
                        if data.fake_conditionally_independent(x, y, sep_set.clone(), &answer) {
                            println!("  INDEP");
                            graph.remove_edge(x, y);
                            sep_sets.insert((x, y), sep_set.into_iter().collect());
                        } else {
                            println!("  NOINDEP");
                        }
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
    use std::time::Instant;
    use rand::prelude::*;
    use crate::{dataframe::DataFrame, graph::{self, Digraph}};

    #[test]
    fn sanity() {
        let nodes: Vec<_> = (0..6).map(|x| x.to_string()).collect();

        let graph = {
            let mut graph = Digraph::unconnected(nodes.clone());
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);
            graph.add_edge(4, 5);
            // 0..5:
            // graph.add_edge(0, 2);
            // graph.add_edge(0, 3);
            // graph.add_edge(1, 3);
            // graph.add_edge(1, 4);
            // graph.add_edge(2, 4);
            graph
        };

        // let graph = crate::structure_learning::fixed_graphs::alarm();
        // let nodes: Vec<_> = (0..graph.len()).map(|x| x.to_string()).collect();


        let df = {
            let mut df = DataFrame::new(nodes.clone());
            df.add_row(nodes.iter().map(|_| 0).collect());
            df
        };


        let (result_pc, _ci_pc) = pc(df.clone(), graph.clone());
        let (result_sc, _ci_sc) = shortcut_pc(df.clone(), graph.clone());
        let (result_pd, _ci_pd) = pc_dual(df.clone(), graph.clone());
        let (result_sd, _ci_sd) = shortcut_pc_dual(df.clone(), graph.clone());

        println!("===========");
        println!("actual: {a:?}\nactual-undirected: {b:?}", b=graph.clone().undirected(),a=graph);
        println!("pc: {:?}", result_pc);
        println!("sc: {:?}", result_sc);
        println!("pd: {:?}", result_pd);
        println!("sd: {:?}", result_sd);

        assert_eq!(result_pc, graph.clone().undirected());
        assert_eq!(result_pc, result_sc);
        assert_eq!(result_pc, result_pd);
        assert_eq!(result_pc, result_sd);
    }
}
