// Imperative solution
// Time complexity - O(n * m)
// Space complexity - O(1)
use std::cmp;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;

        for i in accounts {
            let mut sum = 0;
            for j in i {
                sum += j;
            }

            max = cmp::max(max, sum);
        }

        max
    }
}

// Functional-style solution
// Time complexity - O(n * m)
// Space complexity - O(1)
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|account| account.iter().sum()).max()
    }
}
