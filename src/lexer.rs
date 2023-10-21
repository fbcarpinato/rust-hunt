pub struct Lexer<'a> {
    content: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }

    fn chop(&mut self, n: usize) -> &'a str {
        let (token, rest) = self.content.split_at(n);
        self.content = rest;
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a str
    where
        P: FnMut(char) -> bool,
    {
        let n = self.content.chars().take_while(|&c| predicate(c)).count();
        self.chop(n)
    }

    pub fn next(&mut self) -> Option<String> {
        self.content = self.content.trim_start();

        if self.content.is_empty() {
            return None;
        }

        if self.content.starts_with(char::is_numeric) {
            let token = self.chop_while(char::is_numeric).to_string();
            Some(token)
        } else if self.content.chars().next().unwrap().is_alphabetic() {
            let term = self.chop_while(char::is_alphanumeric).to_lowercase();
            Some(term)
        } else {
            Some(self.chop(1).to_string())
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
