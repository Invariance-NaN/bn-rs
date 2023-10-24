use std::{borrow::BorrowMut, result};

pub fn combinations<T: Copy>(n: usize, xs: &[T]) -> Combinations<T> {
    Combinations { n, xs, indices: None }
}

#[derive(Clone)]
pub struct Combinations<'a, T> {
    n: usize,
    xs: &'a [T],
    indices: Option<Vec<usize>>
}

impl<'a, T: Copy> Iterator for Combinations<'a, T> {
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


pub fn interleave<T, Xs: Iterator<Item = T>, Ys: Iterator<Item = T>>(xs: Xs, ys: Ys) -> Interleave<T, Xs, Ys> {
    Interleave { xs, ys, state: InterleaveState::XsNext }
}

#[derive(Clone, Copy)]
enum InterleaveState {
    XsNext,
    YsNext,
    Done
}

#[derive(Clone, Copy)]
pub struct Interleave<T, Xs: Iterator<Item = T>, Ys: Iterator<Item = T>> {
    xs: Xs,
    ys: Ys,
    state: InterleaveState
}

impl<T, Xs: Iterator<Item = T>, Ys: Iterator<Item = T>> Iterator for Interleave<T, Xs, Ys> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use InterleaveState::{XsNext, YsNext, Done};

        let item = match self.state {
            XsNext => self.xs.next(),
            YsNext => self.ys.next(),
            Done => None
        };

        self.state = match (self.state, &item) {
            (_, None) => Done,
            (Done, _) => Done,
            (XsNext, _) => YsNext,
            (YsNext, _) => XsNext
        };

        item
    }
}
