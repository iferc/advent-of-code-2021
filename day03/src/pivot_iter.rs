pub fn collect_pivoted<T>(
    collection: T,
) -> Vec<Vec<<<T as IntoIterator>::Item as IntoIterator>::Item>>
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    let mut rows = collection
        .into_iter()
        .map(|row| row.into_iter())
        .collect::<Vec<_>>();

    let mut cols = Vec::new();
    loop {
        let col = rows
            .iter_mut()
            .map(|row| row.next())
            .collect::<Option<Vec<<<T as IntoIterator>::Item as IntoIterator>::Item>>>();

        match col {
            Some(col) => cols.push(col),
            None => return cols,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn swaps_columns_and_rows() {
        let given = vec![
            // stop fmt
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        let expected = vec![
            // stop fmt
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
        ];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn empty_iters_to_empty_vec() {
        let given: Vec<Vec<i32>> = vec![vec![]];
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn no_perceived_effect_on_single_element() {
        let given = vec![vec![1]];
        let expected = vec![vec![1]];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn single_child_vec() {
        let given = vec![vec![1, 2]];
        let expected = vec![vec![1], vec![2]];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn multiple_child_vecs() {
        let given = vec![vec![1], vec![2]];
        let expected = vec![vec![1, 2]];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn truncates_to_first_none_in_row() {
        let given = vec![
            // stop fmt
            vec![1, 2, 3, 4],
            vec![1, 2],
        ];
        let expected = vec![
            // stop fmt
            vec![1, 1],
            vec![2, 2],
        ];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn truncates_to_first_none_in_row_again() {
        let given = vec![
            // stop fmt
            vec![1, 2],
            vec![1, 2, 3, 4],
        ];
        let expected = vec![
            // stop fmt
            vec![1, 1],
            vec![2, 2],
        ];

        assert_eq!(expected, collect_pivoted(given));
    }
}
