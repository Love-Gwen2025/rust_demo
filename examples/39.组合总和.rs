/*
 * @lc app=leetcode.cn id=39 lang=rust
 * @lcpr version=30307
 *
 * [39] 组合总和
 */


// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(0, target, &candidates, &mut current, &mut result);
        result
    }

    fn backtrack(
        start: usize,
        remaining: i32,
        candidates: &[i32],
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>
    ) {
        // 找到有效组合
        if remaining == 0 {
            result.push(current.clone());
            return;
        }

        // 剪枝：剩余值为负数，无需继续
        if remaining < 0 {
            return;
        }

        // 遍历从start开始的所有候选数字
        for i in start..candidates.len() {
            let num = candidates[i];

            // 剪枝优化：如果当前数字已经大于剩余值，跳过
            if num > remaining {
                continue;
            }

            current.push(num);
            Self::backtrack(i, remaining - num, candidates, current, result);
            current.pop();
        }
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,3,6,7]\n7\n
// @lcpr case=end

// @lcpr case=start
// [2,3,5]\n8\n
// @lcpr case=end

// @lcpr case=start
// [2]\n1\n
// @lcpr case=end

 */

