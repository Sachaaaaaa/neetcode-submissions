impl Solution {
    pub fn is_valid(s: String) -> bool {

        let mut stack = Vec::with_capacity(s.len());

        for letter in s.bytes(){
            if letter == b'(' || letter == b'[' || letter == b'{' {
                stack.push(letter);
            } else {
                match letter{
                    b'}' => {
                        match stack.pop(){
                            Some(b'{') => (),
                            _ => return false,
                        }
                    },
                    b')' => {
                        match stack.pop(){
                            Some(b'(') => (),
                            _ => return false,
                        }
                    },
                    b']' => {
                        match stack.pop(){
                            Some(b'[') => (),
                            _ => return false,
                        }
                    },
                    _ => (),
                }
            }
        }   
    
        stack.is_empty()
    }
}
