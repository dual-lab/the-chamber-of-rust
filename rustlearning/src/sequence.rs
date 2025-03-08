use std::collections::HashMap;

pub fn find_median(seq: &[i32]) -> Option<i32> {
    let mut cseq = Vec::from(seq);
    cseq.sort_unstable();
    cseq.get(cseq.len() / 2).copied()
}

pub fn find_mode(seq: &[i32]) -> HashMap<i32,i32> {
    let mut mode: HashMap<i32, i32> = HashMap::new();
    for z in seq {
        let count = mode.entry(*z).or_insert(0);
        *count += 1;
    }
    mode
}
