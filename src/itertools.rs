// trait IndexFilterable: Iterator {
//     fn filter_indices(predicate: impl Fn(usize) -> bool) -> FilterIndices<Self::Item>;
// }

struct FilterIndices<I, P> {
    iter: I,
    predicate: P,
    index: usize
}

impl<I: Iterator, P: Fn(usize) -> bool> Iterator for FilterIndices<I, P> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            if (self.predicate)(self.index) {
                self.index += 1;
                return Some(x)
            } else {
                self.index += 1;
            }
        }

        None
    }
}