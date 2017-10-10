// 字母
pub fn is_alpha(c: &char) -> bool {
    match *c {
        'A'...'Z' => true,
        'a'...'z' => true,
        '_' => true,
        _ => false,
    }
}
// 数字
pub fn is_number(c: &char) -> bool {
    match *c {
        '0'...'9' => true,
        _ => false,
    }
}
pub fn is_space(c: &char) -> bool {
    match *c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

#[derive(Debug, Default, Clone)]
pub struct Msg {
    pub chars: Vec<char>,
    pub next: usize,
}

impl Msg {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Msg {
            chars: msg.as_ref().chars().collect(),
            next: 0,
        }
    }
    pub fn peek(&self) -> Option<&char> {
        if self.next + 1 <= self.chars.len() {
            Some(&self.chars[self.next])
        } else {
            None
        }
    }
    pub fn parse_number(&mut self, c: &char) -> Result<f64, String> {
        // 从str里parse尽量多的字节作为f64
        let colon_count = if *c == '.' { 1 } else { 0 };
        let mut s = String::new();
        if *c == '.' && self.peek().is_none() {
            Err(format!("\"{}{}\" starts_with a invalid number!", c, self))?;
        } else if *c == '.' && is_number(self.peek().unwrap()) {
            s.push('0');
            s.push(*c);
        } else if *c == '.' && self.peek().is_some() {
            Err(format!("\"{}{}\" starts_with a invalid number!", c, self))?;
        } else {
            s.push(*c);
        }

        loop {
            if self.peek().is_none() ||
                self.peek() != Some(&&'.') && !is_number(self.peek().unwrap())
            {
                break;
            }
            let cc = self.next().unwrap();
            if cc == '.' && colon_count >= 1 {
                Err(format!("\"{}{}\" contains a invalid number!", s, self))?;
            } else {
                s.push(cc);
            }
        }
        Ok(s.parse::<f64>().unwrap())
    }
}

use std::fmt;
impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut idx = self.next;
        loop {
            if idx + 1 > self.chars.len() {
                break;
            }
            write!(f, "{}", self.chars[idx])?;
            idx += 1;
        }
        Ok(())
    }
}

impl Iterator for Msg {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        if next + 1 <= self.chars.len() {
            self.next += 1;
            Some(self.chars[next])
        } else {
            None
        }
    }
}