impl Solution {
    pub fn count_bits_aux(mut input: i32) -> i32{
        let mut count = 0;
        let mut i = 0;
        while input != 0{
            input &= input - 1;
            count +=1;
        }

        count
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result:  Vec<i32> = Vec::new(); 
        for i in 0..=n{
            let mut input = n;
            result.push(Solution::count_bits_aux(i));
        }
        result
    }
}
