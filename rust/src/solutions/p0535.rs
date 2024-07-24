pub struct Codec {}

impl Codec {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {}
    }

    #[allow(dead_code, non_snake_case)]
    fn encode(&self, longURL: String) -> String {
        longURL
    }

    #[allow(dead_code, non_snake_case)]
    fn decode(&self, shortURL: String) -> String {
        shortURL
    }
}
