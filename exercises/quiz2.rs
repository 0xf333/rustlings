// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter(){
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => output.push(make_it_uppercase(string)),
                Command::Trim => output.push(make_it_trim(string)),
                Command::Append(n) => output.push(make_it_append(string, *n))
            }
        }
        output
    }

    // Helper functions
    fn make_it_uppercase(word: &str) -> String {
        return word.to_uppercase().to_string();
    }

    fn make_it_trim(word: &str) -> String {
        return word.trim().to_string();
    }

    fn make_it_append(word: &str, repeat_n_times: usize) -> String {
        let word_to_append = "bar";
        return format!("{}{}", word.to_string(), word_to_append.repeat(repeat_n_times));
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
