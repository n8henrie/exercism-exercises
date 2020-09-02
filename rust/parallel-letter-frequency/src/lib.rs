use std::collections::HashMap;
use std::thread;

fn char_frequency(input: String) -> HashMap<char, usize> {
    let mut hm = HashMap::new();
    for c in input.chars() {
        if !c.is_alphabetic() {
            continue;
        }
        let c = c.to_ascii_lowercase();
        let count = hm.entry(c).or_insert(0);
        *count += 1
    }
    hm
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("");
    let len = input.len();
    let mut iter = input.chars();

    let mut join_handles = Vec::with_capacity(worker_count);
    let take_length = (len / worker_count).max(1);
    for _ in 0..worker_count {
        let chunk = iter.by_ref().take(take_length).collect::<String>();
        let handle = thread::spawn(move || char_frequency(chunk));
        join_handles.push(handle);
    }

    let mut hm = HashMap::new();
    for handle in join_handles {
        for (k, v) in handle.join().unwrap().into_iter() {
            let counts = hm.entry(k).or_insert(0);
            *counts += v;
        }
    }
    hm
}
