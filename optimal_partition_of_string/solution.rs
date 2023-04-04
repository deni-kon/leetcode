impl Solution {
    pub fn partition_string(s: String) -> i32 {
        // convert String into a byte slice
        // e.g. "abc" will become [97, 98, 99]
        let bytes = s.as_bytes();
        // min number of substrings starting at 1
        let mut cnt: i32 = 1;
        // array with a bool value for 26 letter of the English alphabet
        // true represents the letter has been seen, false - hasn't
        let mut arr: [bool; 26] = [false; 26];

        // iterate over an array, if the value wasn't seen, change to true
        // if it was seen, then increment cnt and reset the array
        for i in 0..bytes.len() {
            if !arr[(bytes[i] as usize) - 97] {
                arr[(bytes[i] as usize) - 97] = true;
            } else {
                cnt += 1;
                arr = [false; 26];
                arr[(bytes[i] as usize) - 97] = true;
            }
        }

        cnt
    }
}
