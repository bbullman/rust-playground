use std::vec::Vec;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        Self::backtrack(&mut results, &mut vec![], n, k, 1);
        results
    }

    pub fn backtrack(results: &mut Vec<Vec<i32>>, 
                    combo: &mut Vec<i32>, 
                    remainder: i32,
                    k: i32, 
                    start: i32) -> () 
    {
        if remainder == 0 && combo.len() as i32 == k {
            results.push(combo.to_vec());
        } else if remainder < 0 || combo.len() as i32 == k {
            return;
        }
        for i in start..=9 {
            combo.push(i);
            Self::backtrack(results, combo, remainder - i, k, i + 1);
            combo.pop();
        }
    }
}
