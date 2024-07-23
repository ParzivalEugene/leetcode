pub struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn encode(&self, longURL: String) -> String {
        longURL
    }

    fn decode(&self, shortURL: String) -> String {
        shortURL
    }
}
