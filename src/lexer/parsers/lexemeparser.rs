use super::super::slidingwindow::SlidingWindow;
use super::tokenparser::TokenParser;
use super::complextokenparser::ComplexTokenParser;
use super::super::token::Token;

pub trait Parser {
    fn parse(&self, sliding_window: &mut SlidingWindow) -> Token;
}

pub struct LexemeParser {
    token_parser: TokenParser,
    complex_token_parser: ComplexTokenParser
}

impl LexemeParser {
    pub fn new() -> LexemeParser {
        LexemeParser {
            token_parser: TokenParser,
            complex_token_parser: ComplexTokenParser
        }
    }

    pub fn parse(&self, sliding_window: &mut SlidingWindow) -> Token {
        let is_complex = self.complex_token_parser.is_complex(sliding_window.current_character());
        match is_complex {
            true => self.complex_token_parser.parse(sliding_window),
            false => self.token_parser.parse(sliding_window)
        }
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::LexemeParser;
    use super::super::super::token::TokenKind;
    use super::super::super::slidingwindow::SlidingWindow;

    fn parser_helper(source: &str) -> TokenKind {
        let parser = LexemeParser::new();
        let mut phrase = SlidingWindow::new(source);
        parser.parse(&mut phrase).kind
    }

    #[test]
    fn testvarioustokens_varietyoftokentypes_successful() {
        assert_eq!(parser_helper("async"), TokenKind::AsyncKeyword);
        assert_eq!(parser_helper("{"), TokenKind::OpenBrace);
        assert_eq!(parser_helper("Customer"), TokenKind::Identifier("Customer".to_string()));
        assert_eq!(parser_helper("\"some string value\""), TokenKind::StringValue("some string value".to_string()));
    }
}