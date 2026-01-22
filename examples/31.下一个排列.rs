/*
 * @lc app=leetcode.cn id=31 lang=rust
 * @lcpr version=30307
 *
 * [31] 下一个排列
 */

use core::num;

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();

        // 反向找第一个 nums[i] < nums[i+1] 的 i
        // windows(2) 产生相邻的两个元素的窗口
        // rposition 从右往左找第一个满足条件的位置
        let pivot = nums
            .windows(2) // [[a,b], [b,c], [c,d], ...]
            .rposition(|pair| pair[0] < pair[1]); // 从右找第一个 左 < 右

        match pivot {
            None => {
                // 整个序列是递减的，直接反转
                nums.reverse();
            }
            Some(i) => {
                // i 是拐点位置
                let mut j = n - 1;
                //因为i以后都是递减的，所以这样能够找到恰好大一点的j
                while nums[j] <= nums[i] {
                    j -= 1;
                }
                nums.swap(i, j);
                nums[i + 1..].reverse();
            }
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3]\n
// @lcpr case=end

// @lcpr case=start
// [3,2,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,5]\n
// @lcpr case=end

 */
