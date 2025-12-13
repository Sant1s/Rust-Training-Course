// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    if vec.len() < 2 {
        return None;
    }

    let mut first_largest: i32 = i32::MIN;
    let mut second_largest: i32 = i32::MIN;

    for &number in vec {
        if number > first_largest {
            second_largest = first_largest;
            first_largest = number;
        } else if number > second_largest && number < first_largest {
            second_largest = number;
        }
    }

    if second_largest == i32::MIN {
        None
    } else {
        Some(second_largest)
    }
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    if init_sequence.is_empty() {
        return vec![];
    }

    let n = init_sequence.len();
    let mut dp = vec![1; n];
    let mut parent = vec![None; n];

    for i in 1..n {
        for j in 0..i {
            if init_sequence[i] > init_sequence[j] && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                parent[i] = Some(j);
            }
        }
    }

    let mut max_length = 0;
    let mut max_index = 0;
    for i in 0..n {
        if dp[i] > max_length {
            max_length = dp[i];
            max_index = i;
        }
    }

    let mut result = Vec::new();
    let mut current = Some(max_index);

    while let Some(idx) = current {
        result.push(init_sequence[idx]);
        current = parent[idx];
    }

    result.reverse();
    result
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    sentence.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                },
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    use std::collections::HashSet;

    let mut char_set: HashSet<char> = HashSet::new();

    for ch in s.chars().flat_map(|c| c.to_lowercase()) {
        if !char_set.insert(ch) {
            return false;
        }
    }
    true
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut freq_map = HashMap::new();
    for num in nums {
        let count = freq_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut freq_vec: Vec<(i32, i32)> = freq_map.into_iter().collect();

    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut result = Vec::new();
    for i in 0..k.min(freq_vec.len()) {
        result.push(freq_vec[i].0);
    }

    result
}
