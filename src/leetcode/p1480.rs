/// 1480. Running Sum of 1d Array
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .scan(0, |state, e| {
            println!("acc: {}, e: {}", *state, e);
            *state = *state + e;
            Some(*state)
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::leetcode::p1480;

    #[test]
    fn running_sum() {
        let tests = vec![
            (vec![1, 1, 1, 1], vec![1, 2, 3, 4]),
            (vec![], vec![]),
            (vec![1, 2, 3, 4], vec![1, 3, 6, 10]),
        ];

        for (given, exp) in tests {
            assert_eq!(p1480::running_sum(given), exp)
        }
    }
}
