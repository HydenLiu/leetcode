/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 *
 * https://leetcode.com/problems/search-insert-position/description/
 *
 * algorithms
 * Easy (44.98%)
 * Likes:    15400
 * Dislikes: 677
 * Total Accepted:    2.6M
 * Total Submissions: 5.8M
 * Testcase Example:  '[1,3,5,6]\n5'
 *
 * Given a sorted array of distinct integers and a target value, return the
 * index if the target is found. If not, return the index where it would be if
 * it were inserted in order.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums contains distinct values sorted in ascending order.
 * -10^4 <= target <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // Submit 1
        // let mut res = nums.len();
        // for i in 0..nums.len() {
        //     if nums[i] >= target {
        //         res = i;
        //         break;
        //     }
        // }
        // res as i32

        // Submit 2
        nums.binary_search(&target).unwrap_or_else(|i| i) as i32
    }
}
// @lc code=end
