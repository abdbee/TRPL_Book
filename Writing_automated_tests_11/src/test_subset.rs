pub fn add_two(x: u32, y:u32)-> u32 {
    x + y
 }

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn is_it_equal1() {
        assert_eq!(4, add_two(2,2));
    }

    #[test]
    #[ignore] //this ignores this test when running tests
    fn is_it_equal2() {
        let value = 4;
        assert_eq!(value, add_two(2,3), "{} was printed instead of {}", value, add_two(2,3));
    }
}

//to only run a particular test, do: cargo test <test_function_name>
// you can specify part of the test name, and any tests that have that part gets run eg, cargo test is

// to ignore a test while running tests, add the "ignore attribute" before writing the test
// to test only ignored tests, run: cargo test -- --ignored