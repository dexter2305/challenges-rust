/// Max Consecutive Ones
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut cur_max = 0;
    let mut cur_size = 0;
    for (cur_index, n) in nums.into_iter().enumerate() {
        if n == 1 {
            cur_size += 1;
        } else if n == 0 {
            cur_size = 0;
        }
        if cur_size > cur_max {
            cur_max = cur_size;
        }
    }
    cur_max
}

/// Find Numbers with Even Number of Digits
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .map(|e| {
            let mut n = e;
            let mut c = 0;
            while n >= 10 {
                n = n / 10;
                c = c + 1;
            }
            c + 1
        })
        .filter(|n| *n % 2 == 0)
        .count() as i32
}
pub fn sorted_squares_v1(nums: Vec<i32>) -> Vec<i32> {
    let mut li = 0;
    let mut ri = nums.len() - 1;
    let mut result: Vec<i32> = Vec::new();

    while li <= ri {
        let l = nums.get(li).unwrap();
        let r = nums.get(ri).unwrap();
        if (l * l) < (r * r) {
            result.push(r * r);
            ri = ri - 1;
        } else {
            result.push(l * l);
            li = li + 1;
        }
    }
    result.into_iter().rev().collect()
}

pub fn sorted_squares_v2(nums: Vec<i32>) -> Vec<i32> {
    let mut li: usize = 0;
    let mut ri: usize = nums.len() - 1;
    let mut results = vec![0; nums.len()];
    for index in (0..nums.len()).rev() {
        let ls = nums[li] * nums[li];
        let rs = nums[ri] * nums[ri];
        if ls > rs {
            results[index] = ls;
            li = li + 1;
        } else {
            results[index] = rs;
            ri = ri.checked_sub(1).unwrap_or(0);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use crate::leetcode::p_unindexed;

    #[test]
    fn find_max_consecutive_ones() {
        let tests = vec![
            (vec![1, 1, 1], 3),
            (vec![0, 0, 0, 0, 0], 0),
            (vec![1, 0, 1, 1, 1], 3),
            (vec![1, 1, 0, 1], 2),
            (vec![1, 0, 0, 1], 1),
        ];
        for (ints, exp) in &tests {
            assert_eq!(
                p_unindexed::find_max_consecutive_ones(ints.to_owned()),
                exp.to_owned(),
                "Given {:?}, expected {}",
                ints,
                exp
            )
        }
    }

    #[test]
    fn find_numbers() {
        let tests = vec![
            (vec![12, 345, 2, 6, 7896], 2),
            (vec![1, 2, 3, 4], 0),
            (vec![1234, 50, 50, 1234], 4),
        ];

        for (ints, exp) in &tests {
            assert_eq!(
                p_unindexed::find_numbers(ints.to_owned()),
                exp.to_owned(),
                "Given {:?}, exp {}",
                ints,
                exp
            )
        }
    }

    #[test]
    fn sorted_squares() {
        let tests = vec![
            (vec![-4, -1, 0, 3, 10], vec![0, 1, 9, 16, 100]),
            (vec![1], vec![1]),
        ];
        for (given, exp) in &tests {
            assert_eq!(
                p_unindexed::sorted_squares_v1(given.to_owned()),
                exp.to_owned()
            )
        }
        for (given, exp) in &tests {
            assert_eq!(
                p_unindexed::sorted_squares_v2(given.to_owned()),
                exp.to_owned()
            )
        }
    }
}
