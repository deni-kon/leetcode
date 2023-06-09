// number can be negative, so with a convertion to string the solution is a perfect one-liner
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return x.to_string().chars().rev().eq(x.to_string().chars())
    }
}
