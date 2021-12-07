use std::collections::HashMap;

/// Consumes the collection and immediately builds key value pair collections
/// where the resulting key is whatever the input function returns.
pub fn group_by<K, V>(
    collection: V,
    derive_key: fn(&<V as IntoIterator>::Item) -> K,
) -> impl Iterator<Item = (K, Vec<<V as IntoIterator>::Item>)>
where
    K: Eq + std::hash::Hash,
    V: IntoIterator,
{
    let mut map = HashMap::new();

    for item in collection {
        let key = derive_key(&item);
        let grouping = map.entry(key).or_insert_with(|| Vec::new());
        grouping.push(item);
    }

    map.into_iter().map(|(key, values)| (key, values))
}
