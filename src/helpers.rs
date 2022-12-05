use std::cmp::{Eq, Ord, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub fn vec_of_strings(input: &str) -> Vec<&str> {
    input.split('\n').map(|str| str.trim()).collect()
}

pub fn vec_of_numbers(input: &str) -> Vec<u32> {
    let strings = vec_of_strings(input);
    strings.iter().map(|str| str.parse().unwrap()).collect()
}

pub fn get_alphabet() -> Vec<String> {
    (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
}

pub fn count<T>(array: &Vec<T>, k: usize) -> Vec<(usize, &T)>
where
    T: Hash + Eq + Ord,
{
    let mut map = HashMap::with_capacity(array.len());

    for x in array {
        *map.entry(x).or_default() += 1;
    }

    let mut heap = BinaryHeap::with_capacity(k + 1);

    for (x, count) in map.into_iter() {
        if heap.len() < k {
            heap.push(Reverse((count, x)));
        } else {
            let &Reverse((min, _)) = heap.peek().unwrap();
            if min < count {
                heap.pop();
                heap.push(Reverse((count, x)));
            }
        }
    }

    heap.into_sorted_vec().into_iter().map(|r| r.0).collect()
}

pub fn parse_str_to_num(num: &str) -> u32 {
    match num.parse::<u32>() {
        Ok(n) => n,
        Err(_) => 0,
    }
}

pub fn parse_char_to_num(num: &char) -> u32 {
    match num.to_string().parse::<u32>() {
        Ok(n) => n,
        Err(_) => 0,
    }
}

pub fn parse_str_to_usize(num: &str) -> usize {
    match num.parse::<usize>() {
        Ok(n) => n,
        Err(_) => 0,
    }
}
