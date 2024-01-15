/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
 *
 * https://leetcode.com/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (55.76%)
 * Likes:    14781
 * Dislikes: 360
 * Total Accepted:    1.9M
 * Total Submissions: 3.3M
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * Given the root of a binary tree, check whether it is a mirror of itself
 * (i.e., symmetric around its center).
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,2,3,4,4,3]
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,2,2,null,3,null,3]
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 1000].
 * -100 <= Node.val <= 100
 *
 *
 *
 * Follow up: Could you solve it both recursively and iteratively?
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_same_tree(
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    let pb = p.borrow();
                    let qb = q.borrow();
                    pb.val == qb.val
                        && is_same_tree(pb.left.clone(), qb.right.clone())
                        && is_same_tree(pb.right.clone(), qb.left.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }
        is_same_tree(
            root.clone().unwrap().borrow().left.clone(),
            root.unwrap().borrow().right.clone(),
        )
    }
}
// @lc code=end
