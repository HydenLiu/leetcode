/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 *
 * https://leetcode.com/problems/contains-duplicate/description/
 *
 * algorithms
 * Easy (61.58%)
 * Likes:    11778
 * Dislikes: 1284
 * Total Accepted:    3.9M
 * Total Submissions: 6.3M
 * Testcase Example:  '[1,2,3,1]'
 *
 * Given an integer array nums, return true if any value appears at least twice
 * in the array, and return false if every element is distinct.
 *
 *
 * Example 1:
 * Input: nums = [1,2,3,1]
 * Output: true
 * Example 2:
 * Input: nums = [1,2,3,4]
 * Output: false
 * Example 3:
 * Input: nums = [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 *
 *
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // 1
        // let mut map = HashMap::new();
        // for (i, num) in nums.iter().enumerate() {
        //     if let Some(j) = map.insert(num, i) {
        //         return true;
        //     }
        // }
        // false

        // 2
        if nums.len() < 2 {
            return false;
        }

        use std::colletions::HashSet;
        nums.iter().collect::<HashSet<_>>().len() != nums.len()
    }
}
// @lc code=end
