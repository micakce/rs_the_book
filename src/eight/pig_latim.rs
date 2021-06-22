fn ay_latim(word : &str) -> String {
    return format!("{}-{}ay", &word[1..], &word[..1]);
}

fn hay_latim(word : &str) -> String {
    return format!("{}-hay", &word);
}

pub fn str2pig_latim(word : &str) -> String {
    match &word[..1] {
        "a" | "e" | "i" | "o" | "u" => hay_latim(word),
        _ => ay_latim(word)
    }

}
