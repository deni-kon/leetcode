impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut sorted_people = people.clone();
        sorted_people.sort();
        let mut boats = 0;
        let mut left: usize = 0;
        let mut right: usize = sorted_people.len() - 1;
        
        while left <= right {
            if right == 0 {
                return boats + 1
            }
            if sorted_people[left] + sorted_people[right] <= limit {
                left += 1;
            }
            right -= 1;
            boats += 1;
        }
        
        boats
    }
}
