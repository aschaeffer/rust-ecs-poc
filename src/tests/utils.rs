use random_string::{RandomString, Charset, Charsets};

pub fn r_string() -> String {
    RandomString::generate(
        10,
        &Charset::from_charsets(Charsets::Letters)
    ).to_string()
}
