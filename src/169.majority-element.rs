/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 *
 * https://leetcode.com/problems/majority-element/description/
 *
 * algorithms
 * Easy (64.78%)
 * Likes:    18716
 * Dislikes: 591
 * Total Accepted:    2.8M
 * Total Submissions: 4.3M
 * Testcase Example:  '[3,2,3]'
 *
 * Given an array nums of size n, return the majority element.
 *
 * The majority element is the element that appears more than ⌊n / 2⌋ times.
 * You may assume that the majority element always exists in the array.
 *
 *
 * Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 *
 *
 * Constraints:
 *
 *
 * n == nums.length
 * 1 <= n <= 5 * 10^4
 * -10^9 <= nums[i] <= 10^9
 *
 *
 *
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 */

// @lc code=start
impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        // 1、
        // let mut count = 0;
        // let mut m = 0;
        // for i in nums {
        //     if count == 0 {
        //         m = i;
        //         count = 1;
        //     } else if i == m {
        //         count += 1;
        //     } else {
        //         count -= 1;
        //     }
        // }
        // m

        // 2
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}
// @lc code=end
