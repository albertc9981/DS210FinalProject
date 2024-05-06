use std::collections::HashSet;

pub fn calculate_jaccard_similarity<T: Eq + std::hash::Hash>(set1: &HashSet<T>, set2: &HashSet<T>) -> f64 {
    let intersection: HashSet<_> = set1.intersection(set2).collect();
    let union: HashSet<_> = set1.union(set2).collect();
    intersection.len() as f64 / union.len() as f64
}