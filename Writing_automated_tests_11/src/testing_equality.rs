//assert_eq! (pass if left = right) and assert_ne! (pass if left != right) macros compare two arguments for equality and inequality repsectively
//these have an advantage over assert_eq because it prints both values if the test fails

pub fn adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn did_it_add(){
        assert_eq!(6, adder(3,3)) // assert that the adder function produces the expected value
    }
}

// the order of the arguments in the assert macros don't matter. the paramters are just called "left" and "right" (unlike in some
// languages where it's called "expected" and "actual"). This means that the function to be tested could come in the left or right
// argument. The position won't matter.
//assert_ne! is much more useful when we're not sure what the value will be, but we know what the value will not be.
//

