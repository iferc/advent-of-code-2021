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
    pub fn example1() {
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
    pub fn example2() {
        let given: Vec<Vec<i32>> = vec![vec![]];
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn example3() {
        let given: Vec<Vec<i32>> = vec![vec![1]];
        let expected: Vec<Vec<i32>> = vec![vec![1]];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn example4() {
        let given: Vec<Vec<i32>> = vec![vec![1, 2]];
        let expected: Vec<Vec<i32>> = vec![vec![1], vec![2]];

        assert_eq!(expected, collect_pivoted(given));
    }

    #[test]
    pub fn example5() {
        let given: Vec<Vec<i32>> = vec![vec![1], vec![2]];
        let expected: Vec<Vec<i32>> = vec![vec![1, 2]];

        assert_eq!(expected, collect_pivoted(given));
    }
}
