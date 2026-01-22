



fn main (){
    let nums= [1,2,3,4];
    let target=4;
    let result= binary_search(&nums, target);
    print!("result:{}",result);
}
fn binary_search(nums: &[i32], target: i32) -> i32 {
    //闭区间写法
    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2;  // 防止溢出
        
        if nums[mid as usize] == target {
            return mid;  // 找到目标，返回索引
        } else if nums[mid as usize] < target {
            left = mid + 1;  // 目标在右半部分
        } else {
            right = mid - 1;  // 目标在左半部分
        }
    }
    
    -1  // 未找到
}