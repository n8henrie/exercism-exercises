pub fn twofer(name: &str) -> String {
    let response = match name {
        "" => "you",
        n => n,
    };
    format!("One for {}, one for me.", response)
}
