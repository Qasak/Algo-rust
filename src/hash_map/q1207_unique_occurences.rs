use std::collections::HashMap;

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    // val freq
    let mut cnt:HashMap<i32, usize> = HashMap::new();
    // freq val
    let mut r = vec![false; 1001];
    for i in arr {
        *cnt.entry(i).or_insert(0) += 1;
    }
    for e in cnt {
        let f = e.1;
        if r[f] {
            return false;
        }
        r[f] = true;
    }
    true
}