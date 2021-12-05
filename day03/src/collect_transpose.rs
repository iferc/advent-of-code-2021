pub fn collect_transposed<T>(
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

/// Tests with integer contents.
#[cfg(test)]
mod copyable_tests {
    use super::*;

    #[test]
    pub fn swaps_columns_and_rows() {
        let given = vec![
            // original rows of columns
            vec![1, 2, 3, 4],
            vec![11, 12, 13, 14],
        ];
        let expected = vec![
            // resulting columns of rows
            vec![1, 11],
            vec![2, 12],
            vec![3, 13],
            vec![4, 14],
        ];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn empty_iters_to_empty_vec() {
        let given: Vec<Vec<i32>> = vec![vec![]];
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn no_perceived_effect_on_single_element() {
        let given = vec![vec![1]];
        let expected = vec![vec![1]];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn single_child_vec() {
        let given = vec![vec![1, 2]];
        let expected = vec![vec![1], vec![2]];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn multiple_child_vecs() {
        let given = vec![vec![1], vec![2]];
        let expected = vec![vec![1, 2]];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn truncates_to_first_none_in_row() {
        let given = vec![
            // original rows of columns
            // resulting columns of rows
            // stop fmt
            vec![1, 2, 3, 4],
            vec![11, 12],
        ];
        let expected = vec![
            // resulting columns of rows
            // stop fmt
            vec![1, 11],
            vec![2, 12],
        ];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn truncates_to_first_none_in_row_again() {
        let given = vec![
            // stop fmt
            vec![1, 2],
            vec![11, 12, 13, 14],
        ];
        let expected = vec![
            // stop fmt
            vec![1, 11],
            vec![2, 12],
        ];

        assert_eq!(expected, collect_transposed(given));
    }
}

/// Tests with String contents so that there is no Copy behaviour accidentally going on.
#[cfg(test)]
mod non_copyable_tests {
    use super::*;

    #[test]
    pub fn swaps_columns_and_rows() {
        let given = vec![
            // stop fmt
            vec![format!("1"), format!("2"), format!("3"), format!("4")],
            vec![format!("11"), format!("12"), format!("13"), format!("14")],
        ];
        let expected = vec![
            // stop fmt
            vec![format!("1"), format!("11")],
            vec![format!("2"), format!("12")],
            vec![format!("3"), format!("13")],
            vec![format!("4"), format!("14")],
        ];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn empty_iters_to_empty_vec() {
        let given: Vec<Vec<String>> = vec![vec![]];
        let expected: Vec<Vec<String>> = vec![];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn no_perceived_effect_on_single_element() {
        let given = vec![vec![format!("1")]];
        let expected = vec![vec![format!("1")]];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn single_child_vec() {
        let given = vec![vec![format!("1"), format!("2")]];
        let expected = vec![vec![format!("1")], vec![format!("2")]];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn multiple_child_vecs() {
        let given = vec![vec![format!("1")], vec![format!("2")]];
        let expected = vec![vec![format!("1"), format!("2")]];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn truncates_to_first_none_in_row() {
        let given = vec![
            // stop fmt
            vec![format!("1"), format!("2"), format!("3"), format!("4")],
            vec![format!("11"), format!("12")],
        ];
        let expected = vec![
            // stop fmt
            vec![format!("1"), format!("11")],
            vec![format!("2"), format!("12")],
        ];

        assert_eq!(expected, collect_transposed(given));
    }

    #[test]
    pub fn truncates_to_first_none_in_row_again() {
        let given = vec![
            // stop fmt
            vec![format!("1"), format!("2")],
            vec![format!("11"), format!("12"), format!("13"), format!("14")],
        ];
        let expected = vec![
            // stop fmt
            vec![format!("1"), format!("11")],
            vec![format!("2"), format!("12")],
        ];

        assert_eq!(expected, collect_transposed(given));
    }
}
