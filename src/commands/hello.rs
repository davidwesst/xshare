/// Return a greeting using the name, if provided.
pub fn say_hello(name: Option<String>) -> String {
    let default_message =  "Hello there! I don't know your name, but I know you are amazing because you are using XShare.";

    match name {
        Some(ref name_value) if name_value.is_empty() => {
           default_message.to_string()
        },
        Some(name_value) => format!("Hello {}! Thank you for using XShare.", name_value),
        None => {
            default_message.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello_with_name() {
        // arrange
        let expected_value = "DW";

        // act
        let result = say_hello(Some(expected_value.to_string()));

        // assert
        assert!(
            result.contains(expected_value),
            "Expected hello message to contain '{}'",
            expected_value
        );
    }

    #[test]
    fn test_say_hello_without_name() {
        // arrange
        let expected_substring = "Hello there";

        // act
        let result = say_hello(None);

        // assert
        assert!(
            result.contains(expected_substring),
            "Expected hello messge to contain '{}'",
            expected_substring
        );
    }

    #[test]
    fn test_say_hello_with_empty_string() {
        // arrange
        let expected_substring = "Hello there";

        // act
        let result = say_hello(Some(String::new()));

        // assert
        assert!(
            result.contains(expected_substring),
            "Expected hello message to contain '{}'",
            expected_substring
        );
    }
}
