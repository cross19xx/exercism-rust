/// Check if a string is all uppercase
/// This also accounts for edge cases where a string may not contain a letter at all
fn is_all_uppercase(input: &str) -> bool {
    let mut has_alphabetic = false;

    for char in input.chars() {
        if char.is_alphabetic() {
            has_alphabetic = true;

            if !char.is_uppercase() {
                return false;
            }
        }
    }

    has_alphabetic
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    let is_silence = message.is_empty();
    let is_yelling = !is_silence && is_all_uppercase(message);
    let is_question = message.ends_with('?');

    println!("{} {} {}", is_silence, is_yelling, is_question);

    if !is_yelling && is_question {
        "Sure."
    } else if is_yelling && !is_question {
        "Whoa, chill out!"
    } else if is_yelling && is_question {
        "Calm down, I know what I'm doing!"
    } else if is_silence {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
