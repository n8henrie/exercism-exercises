const QUESTION_RESPONSE: &str = "Sure.";
const YELLING_RESPONSE: &str = "Whoa, chill out!";
const YELLING_QUESTION_RESPONSE: &str = "Calm down, I know what I'm doing!";
const SILENCE_RESPONSE: &str = "Fine. Be that way!";
const DEFAULT_RESPONSE: &str = "Whatever.";

pub fn is_yelling(m: &str) -> bool {
    m.contains(char::is_alphabetic) && m.chars().all(|c| c.is_uppercase() || !c.is_alphabetic())
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => SILENCE_RESPONSE,
        m if m.ends_with('?') => {
            if is_yelling(m) {
                YELLING_QUESTION_RESPONSE
            } else {
                QUESTION_RESPONSE
            }
        }
        m if is_yelling(m) => YELLING_RESPONSE,
        _ => DEFAULT_RESPONSE,
    }
}
