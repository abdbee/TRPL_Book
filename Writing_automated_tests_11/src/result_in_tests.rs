
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn works() -> Result<(), String> {
        if 2 + 2 == 5{
            Ok(())
        }
        else {
            Err(String::from("two plus 2 is not equal to 4"))
        }
    }

}

//note that you can't ue the #[should_panic] annotation on tests that use the Result<T, E>.