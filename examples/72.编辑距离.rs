/*
 * @lc app=leetcode.cn id=72 lang=rust
 * @lcpr version=30307
 *
 * [72] 编辑距离
 */

// @lc code=start
impl Solution {
    /// 编辑距离（Levenshtein Distance）
    ///
    /// # 算法思路
    /// 动态规划：dp[i][j] 表示 word1[0..i] 变成 word2[0..j] 的最少操作数
    ///
    /// # 状态转移
    /// - 字符相同：dp[i][j] = dp[i-1][j-1]
    /// - 字符不同：dp[i][j] = min(插入, 删除, 替换) + 1
    ///
    /// # 时间复杂度：O(m * n)
    /// # 空间复杂度：O(m * n)
    pub fn min_distance(word1: String, word2: String) -> i32 {
        //dp[i][j]定义为，将word1[0..i]（前i个，不包括i）变成word2[0..j]所需的最小步数
        //这里分四种情况
        //1.word1[i-1]=word2[j-1]  那么只用考虑dp[i-1][j-1]
        //2.word1[i-1]!=word2[j-1]
        //  考虑2.1 : 那么考虑如果最后一步是替换，那么状态一定是从dp[i-1][j-1]转来的，因为不然替换操作就不会使状态变为dp[i][j]
        //2.2,2.3同理

        // 缺少这些
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // 缺少边界初始化
        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                if word1[i-1] == word2[j-1] {
                    dp[i][j] = dp[i - 1][j - 1] //不用操作
                } else {
                    dp[i][j] = 1 + std::cmp::min(
                        dp[i - 1][j],
                        std::cmp::min(dp[i - 1][j - 1], dp[i][j - 1]),
                    )
                }
            }
        }

        dp[m][n]
    }
}
// @lc code=end

/*
// @lcpr case=start
// "horse"\n"ros"\n
// @lcpr case=end

// @lcpr case=start
// "intention"\n"execution"\n
// @lcpr case=end

 */
