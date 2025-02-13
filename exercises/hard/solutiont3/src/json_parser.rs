use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub struct JsonParseError {
    message: String,
    position: usize,
}

pub struct JsonParser<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> JsonParser<'a> {
    pub fn new(input: &'a str) -> Self {
        JsonParser {
            chars: input.chars().peekable(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, JsonParseError> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonParseError> {
        match self.chars.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => self.parse_string(),
            Some('t') | Some('f') => self.parse_boolean(),
            Some('n') => self.parse_null(),
            Some(c) if c.is_ascii_digit() || *c == '.' || *c == '-' || *c == 'e' || *c == 'E' => {
                self.parse_number()
            }
            _ => Err(self.error("Unexpected character")),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonParseError> {
        self.expect_char('{')?;
        let mut map = HashMap::new();

        loop {
            self.skip_whitespace();
            if self.next_char_is('}') {
                break;
            }

            let key = self.parse_string()?;
            if let JsonValue::String(s) = key {
                self.skip_whitespace();
                self.expect_char(':')?;
                self.skip_whitespace();
                let value = self.parse_value()?;
                map.insert(s, value);

                self.skip_whitespace();
                if !self.next_char_is(',') {
                    if self.chars.peek() == Some(&'}') {
                        break;
                    } else {
                        return Err(self.error("Expected ',' or '}'"));
                    }
                }
                self.chars.next();
                self.position += 1;
            } else {
                return Err(self.error("Expected string key"));
            }
        }

        self.expect_char('}')?;
        Ok(JsonValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonParseError> {
        self.expect_char('[')?;
        let mut arr = Vec::new();

        loop {
            self.skip_whitespace();
            if self.next_char_is(']') {
                break;
            }

            let value = self.parse_value()?;
            arr.push(value);

            self.skip_whitespace();
            if !self.next_char_is(',') {
                if self.chars.peek() == Some(&']') {
                    break;
                } else {
                    return Err(self.error("Expected ',' or ']'"));
                }
            }
            self.chars.next();
            self.position += 1;
        }

        self.expect_char(']')?;
        Ok(JsonValue::Array(arr))
    }

    fn next_char_is(&mut self, expected: char) -> bool {
        self.chars.peek() == Some(&expected)
    }

    fn parse_string(&mut self) -> Result<JsonValue, JsonParseError> {
        let mut s = String::new();
        self.expect_char('"')?;

        for c in self.chars.by_ref() {
            self.position += 1;
            match c {
                '"' => break,
                _ => s.push(c),
            }
        }

        Ok(JsonValue::String(s))
    }

    fn parse_number(&mut self) -> Result<JsonValue, JsonParseError> {
        let mut num_str = String::new();
        let mut special_char_vec = Vec::new();
        let special_chars = ['.', '-', 'e', 'E'];

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() || special_chars.contains(c) {
                if special_chars.contains(c) {
                    if special_char_vec.contains(c) {
                        return Err(self.error("Invalid number format"));
                    }
                    special_char_vec.push(*c);
                }
                num_str.push(*c);
                self.chars.next();
                self.position += 1;
            } else {
                break;
            }
        }

        num_str
            .parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| self.error("Invalid number format"))
    }

    fn parse_null(&mut self) -> Result<JsonValue, JsonParseError> {
        let word: String = self
            .chars
            .by_ref()
            .take(4) // "null"
            .inspect(|_| self.position += 1)
            .collect();

        if word == "null" {
            Ok(JsonValue::Null)
        } else {
            Err(self.error("Invalid null value"))
        }
    }

    fn parse_boolean(&mut self) -> Result<JsonValue, JsonParseError> {
        let word: String = self
            .chars
            .by_ref()
            .take(4) // "true" 或 "fals"（后续再检查）
            .inspect(|_| self.position += 1)
            .collect();

        match word.as_str() {
            "true" => Ok(JsonValue::Boolean(true)),
            "fals" if self.chars.next() == Some('e') => {
                self.position += 1;
                Ok(JsonValue::Boolean(false))
            }
            _ => Err(self.error("Invalid boolean value")),
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), JsonParseError> {
        match self.chars.next() {
            Some(c) if c == expected => {
                self.position += 1;
                Ok(())
            }
            Some(c) => Err(self.error(&format!("Expected '{}', found '{}'", expected, c))),
            None => Err(self.error(&format!("Expected '{}' but reached end", expected))),
        }
    }

    fn error(&self, message: &str) -> JsonParseError {
        JsonParseError {
            message: message.to_string(),
            position: self.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object() {
        let input =
            r#"{"name": "New York", "population": 83999000, "area": 302.61, "capital": "Albany"}"#;
        let mut parser = JsonParser::new(input);
        let json_value = parser.parse().unwrap();
        println!("{:?}", json_value)
    }

    #[test]
    fn test_parse_array() {
        let input = r#"[1, 2, 3, 4, 5]"#;
        let mut parser = JsonParser::new(input);
        let value = parser.parse().unwrap();
        assert_eq!(
            value,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
                JsonValue::Number(4.0),
                JsonValue::Number(5.0),
            ])
        );
    }

    #[test]
    fn test_parse_string() {
        let input = r#""hello, world""#;
        let mut parser = JsonParser::new(input);
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::String("hello, world".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let input = r#"3.14"#;
        let mut parser = JsonParser::new(input);
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Number(3.14));
    }

    #[test]
    fn test_parse_null() {
        let input = r#"null"#;
        let mut parser = JsonParser::new(input);
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Null);
    }

    #[test]
    fn test_parse_boolean() {
        let input = r#"true"#;
        let mut parser = JsonParser::new(input);
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Boolean(true));

        let input = r#"false"#;
        let mut parser = JsonParser::new(input);
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Boolean(false));
    }

    #[test]
    fn test_parse_invalid_input() {
        let input =
            r#"{"name": "New York", "population": 83999000, "area": 302.61, "capital": "Albany""#;
        let mut parser = JsonParser::new(input);
        let err = parser.parse().unwrap_err();
        assert_eq!(err.message, "Expected ',' or '}'");
        assert_eq!(err.position, 80);
    }

    #[test]
    fn test_parse_invalid_number() {
        let input = r#"3.14.1"#;
        let mut parser = JsonParser::new(input);
        let err = parser.parse().unwrap_err();
        assert_eq!(err.message, "Invalid number format");
        assert_eq!(err.position, 4);
    }
}
