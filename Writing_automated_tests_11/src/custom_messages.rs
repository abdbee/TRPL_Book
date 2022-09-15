// print customm messages in case of assertion errors, add a second (for the assert! macro) or a third (for the assert_eq or assert_ne macros)
// argument where you'll right the error, using the pattern used in the values of macros like println! and format!.

pub fn greeting(name: &str) -> String {
    format!("hello {}!", name) //this returns (instead of print) the result
}

#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("basit"); // returns "hello sadiq" to the "result" variable
        let container = "basit";
        assert!(result.contains("container"), "greeetings did not contain \"{}\". The value gotten was \"{}\"", container, result)
    }

    // but what if we want to ensure that our code handles error conditions correctly?
    #[test]
    
    #[should_panic(expected = "greetings should contain result")] //this attribute makes the test pass if the code inside the function panics
    // the "expected" argument  signifies the specific "panic" message we are testing for. the test will fail if this panic message is not gotten after running the function
    fn did_it_panic() {
        let result = greeting("basit"); // returns "hello sadiq" to the "result" variable
        let container = "sadiq";
        assert!(result.contains("container"), "greeetings did not contain \"{}\". The value gotten was \"{}\"", container, result)
    }

}