//assert_eq! (pass if left = right) and assert_ne! (pass if left != right) macros compare two arguments for equality and inequality repsectively
//these have an advantage over assert_eq because it prints both values if the test fails

pub fn adder(a: i32, b: i32) -> i32 {
    a + b
}

//since values are being compared and the assert macros print values in debug formating, all the types must implement both the Debug and PartialEq
//traits. since structs and enums don't implement these by default, you'll have to implement these by adding the annotation as shown in the next line.
#[derive(PartialEq, Debug)] 
pub struct Rectangle {
    pub length: u32,
    pub width: u32
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn test_struct(){
        assert_eq!(Rectangle{length:3, width:3}, Rectangle{length:3, width:3} ) // assert that the adder function produces the expected value
    }

    #[test]
    pub fn does_it_add() {
        assert_eq!(6, adder(3,3))
    }
}

// the order of the arguments in the assert macros don't matter. the paramters are just called "left" and "right" (unlike in some
// languages where it's called "expected" and "actual"). This means that the function to be tested could come in the left or right
// argument. The position won't matter.
//assert_ne! is much more useful when we're not sure what the value will be, but we know what the value will not be.
//if we want to see printed values for passing test as well, we can use cargo test -- --show-output

