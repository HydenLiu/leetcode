/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 *
 * https://leetcode.com/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (73.48%)
 * Likes:    12373
 * Dislikes: 407
 * Total Accepted:    1.5M
 * Total Submissions: 2.1M
 * Testcase Example:  '5'
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 *
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it as shown:
 *
 *
 * Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= numRows <= 30
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn generate(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ret = vec![vec![1]];
        for i in 0..n - 1 {
            let mut p = &ret[i];
            let mut r = vec![1];
            for j in 0..p.len() - 1 {
                r.push(p[j] + p[j + 1]);
            }
            r.push(1);
            ret.push(r);
        }

        ret
    }
}
// @lc code=end
