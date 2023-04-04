impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n & 1 == 0 || n == 1 {
            return n / 2;
        }
        n
    }
}
