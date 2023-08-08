use std::borrow::BorrowMut;

pub fn combinations<T: Copy>(n: usize, xs: Vec<T>) -> Combinations<T> {
    Combinations { n, xs, indices: None }
}

pub struct Combinations<T> {
    n: usize,
    xs: Vec<T>,
    indices: Option<Vec<usize>>
}

impl<T: Copy> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(indices) = self.indices.borrow_mut() {
            let to_modify = if indices.last()? + 1 < self.xs.len() {
                self.n - 1
            } else {
                indices
                    .windows(2)
                    .enumerate()
                    .rev()
                    .find(|(_, index_pair)| index_pair[0] + 1 < index_pair[1])
                    .map(|(i, _)| i)?
            };

            let mut value = indices[to_modify];

            for i in to_modify..self.n {
                value += 1;
                indices[i] = value;
            }

            Some(indices.iter().map(|&index| self.xs[index]).collect())
        } else {
            if self.n > self.xs.len() { return None; }
            let indices = (0..self.n).collect::<Vec<usize>>();
            let result = indices.iter().map(|&index| self.xs[index]).collect();
            self.indices = Some(indices);
            Some(result)
        }
    }
}
