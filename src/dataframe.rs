use std::collections::{HashMap, BTreeSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use crate::graph::Digraph;

pub struct Variable {
    name: String,
    values: usize
}

#[derive(Clone)]
pub struct DataFrame {
    column_names: Vec<String>,
    columns: Vec<Vec<u32>>,
    len: usize
}

impl DataFrame {
    #[inline]
    fn _debug_check_integrity(&self) {
        debug_assert_eq!(self.column_names.len(), self.columns.len());

        // if self.len == 0 {
        //     debug_assert!(self.column_names.is_empty());
        //     debug_assert!(self.columns.is_empty());
        //     debug_assert_eq!(self.len, 0);
        //     return;
        // }

        let column_lens = self.columns.iter().map(|x| x.len()).collect::<std::collections::HashSet<_>>();

        debug_assert_eq!(column_lens.len(), 1);
        debug_assert_eq!(column_lens.into_iter().next(), Some(self.len));
    }

    pub fn new(column_names: Vec<String>) -> Self {
        let columns = vec![vec![]; column_names.len()];

        DataFrame { column_names, columns, len: 0 }
    }

    pub fn names(&self) -> &Vec<String> {
        &self.column_names
    }

    pub fn add_row(&mut self, row: Vec<u32>) {
        self._debug_check_integrity();

        assert!(row.len() == self.columns.len());

        for (x, col) in row.into_iter().zip(self.columns.iter_mut()) {
            col.push(x);
        }

        self.len += 1;

    }

    pub fn len(&self) -> usize {
        self._debug_check_integrity();

        self.len
    }

    pub fn get_row(&self, index: usize) -> Option<Vec<u32>> {
        self._debug_check_integrity();

        let mut result = Vec::with_capacity(self.columns.len());

        for col in &self.columns {
            result.push(*col.get(index)?);
        }

        Some(result)
    }

    pub fn group_by(&self, columns: &Vec<usize>) -> HashMap<Vec<u32>, DataFrame> {
        self._debug_check_integrity();

        let mut result: HashMap<Vec<u32>, DataFrame> = HashMap::new();

        for row in self.iter() {
            let key = columns.iter().map(|&i| row[i]).collect::<Vec<_>>();

            result.entry(key)
                .and_modify(|data_frame| data_frame.add_row(row.clone()))
                .or_insert({
                    let mut data_frame = DataFrame::new(self.column_names.clone());
                    data_frame.add_row(row);
                    data_frame
                });
        }

        result
    }

    pub fn iter(&self) -> Iter<'_> {
        self._debug_check_integrity();

        Iter { data_frame: &self, index: 0 }
    }
}

impl DataFrame {
    pub fn conditionally_independent(&self, x: usize, y: usize, zs: &Vec<usize>) -> bool {
        for (_, group) in self.group_by(zs) {
            let x_vals = group.iter().map(|row| row[x]).collect::<BTreeSet<_>>();
            let y_vals = group.iter().map(|row| row[y]).collect::<BTreeSet<_>>();

            let mut result = 0_f64;

            for x_val in x_vals {
                for &y_val in &y_vals {
                    let x_count  = group.iter().filter(|row| row[x] == x_val).count();
                    let y_count  = group.iter().filter(|row| row[y] == y_val).count();
                    let xy_count = group.iter().filter(|row| row[x] == x_val && row[y] == y_val).count();

                    let expected_xy_count = (x_count * y_count) as f64 / group.len() as f64;

                    let difference = xy_count as f64 - expected_xy_count;

                    result += (difference * difference) / expected_xy_count;
                }
            }

            if result >= 1e8 { return false; }
        }

        return true;
    }

    pub fn fake_conditionally_independent(
        &self,
        x: usize,
        y: usize,
        zs: Vec<usize>,
        graph: &Digraph
    ) -> bool {
        for (_, group) in self.group_by(&zs) {
            let x_vals = group.iter().map(|row| row[x]).collect::<BTreeSet<_>>();
            let y_vals = group.iter().map(|row| row[y]).collect::<BTreeSet<_>>();

            let mut result = 0_f64;

            for x_val in x_vals {
                for &y_val in &y_vals {
                    let x_count  = group.iter().filter(|row| row[x] == x_val).count();
                    let y_count  = group.iter().filter(|row| row[y] == y_val).count();
                    let xy_count = group.iter().filter(|row| row[x] == x_val && row[y] == y_val).count();

                    let expected_xy_count = (x_count * y_count) as f64 / group.len() as f64;

                    let difference = xy_count as f64 - expected_xy_count;

                    result += (difference * difference) / expected_xy_count;
                }
            }

            if result >= 1e12 { return true; }
        }

        // let mut hasher = DefaultHasher::new();
        // x.hash(&mut hasher);
        // y.hash(&mut hasher);
        // zs.hash(&mut hasher);
        // let hash = hasher.finish();

        // if hash < u64::MAX / 50 {
        //     return !graph.d_separated(x, y, zs);
        // }

        return graph.d_separated(x, y, zs);
    }
}


#[derive(Clone, Copy)]
pub struct Iter<'a> {
    data_frame: &'a DataFrame,
    index: usize
}

impl<'a> Iterator for Iter<'a> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.data_frame.get_row(self.index);
        self.index += 1;
        result
    }
}
