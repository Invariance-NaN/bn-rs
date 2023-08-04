pub fn combinations<T: Copy>(n: usize, xs: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    assert!(n <= xs.len());
    
    let mut indices = (0..n).collect::<Vec<usize>>();

    std::iter::once(indices.iter().map(|&index| xs[index]).collect()).chain(
        std::iter::from_fn(move || {
                let to_modify = if indices.last()? + 1 < xs.len() {
                    n - 1
                } else {
                    indices
                        .windows(2)
                        .enumerate()
                        .rev()
                        .find(|(_, index_pair)| index_pair[0] + 1 < index_pair[1])
                        .map(|(i, _)| i)?
                };
                
                let mut value = indices[to_modify];

                for i in to_modify..n {
                    value += 1;
                    indices[i] = value;
                }

            Some(indices.iter().map(|&index| xs[index]).collect())
        })
    )
}

