use std::i32;

/// 1672.richest-customer-wealth

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let sum_of_accounts = accounts
        .iter()
        .map(|amounts| amounts.iter().fold(0, |a, e| a + e))
        .collect::<Vec<i32>>();

    sum_of_accounts.iter().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::leetcode::p1672;

    #[test]
    fn maximum_wealth() {
        let tests = vec![
            (vec![vec![1, 2, 3], vec![3, 2, 1]], 6),
            (vec![vec![1, 5], vec![3, 7], vec![5, 3]], 10),
        ];

        for (given, exp) in tests {
            assert_eq!(p1672::maximum_wealth(given), exp)
        }
    }
}
