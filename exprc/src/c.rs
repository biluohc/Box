use std::collections::BTreeMap as Map;

include!("msg.rs");
#[derive(Debug, Default, Clone)]
pub struct Caculater {
    pub msg: Msg,
    pub current_tok: Token,
    pub recent_name: String,
    pub recent_unassign: String,
    pub recent_number: f64,
    pub name_number_map: Map<String, f64>,
}

impl Caculater {
    pub fn new() -> Self {
        let mut c = Self::default();
        c.name_number_map.insert("pi".to_owned(), 3.1415926535);
        c.name_number_map.insert("e".to_owned(), 2.71828182845);
        c
    }
    pub fn msg_update(&mut self, msg: &str) {
        self.msg = Msg::new(msg);
    }
    pub fn get_token(&mut self) -> Result<(), String> {
        let mut c = '\n'; // tmp value, will be covered
        while c != ' ' && is_space(&c) {
            if self.msg.peek().is_none() {
                self.current_tok = Token::End;
                return Ok(());
            }
            c = self.msg.next().unwrap();
        }

        match c {
            ';' | ' ' => {
                self.current_tok = Token::Print;
                Ok(())
            }
            '+' | '-' | '*' | '/' | '(' | ')' | '=' => {
                self.current_tok = Token::parse(&c).unwrap();
                Ok(())
            }
            cc @ '0'...'9' | cc @ '.' => {
                self.current_tok = Token::Number;
                self.recent_number = self.msg.parse_number(&cc)?;
                Ok(())
            }
            // 变量名以字母/下划线开头
            cc if is_alpha(&cc) => {
                self.recent_name.clear();
                self.recent_name.push(cc);
                loop {
                    if self.msg.peek().is_some() && is_alpha(self.msg.peek().as_ref().unwrap()) {
                        self.recent_name.push(self.msg.next().unwrap())
                    } else {
                        break;
                    }
                }
                self.current_tok = Token::Name;
                if self.name_number_map.get(&self.recent_name).is_none() {
                    self.recent_unassign = self.recent_name.to_owned();
                }
                Ok(())
            }
            _ => Err(format!("{}{} starts with a invalid Token!", c, self.msg)),
        }
    }
    pub fn prim(&mut self, get: bool) -> Result<f64, String> {
        if get {
            self.get_token()?; //读取下一个元素
        }
        match self.current_tok {
            Token::Number => {
                // 取出当前数值
                let value = self.recent_number;
                // 再读一个元素
                self.get_token()?;
                Ok(value)
            }
            Token::Name => {
                let mut value = self.name_number_map.get(&self.recent_name).cloned();
                self.get_token()?;
                if self.current_tok == Token::Assign {
                    value = Some(self.expr(true)?);
                }
                if value.is_none() {
                    return Err(format!("variable: '{}' is undefined !", &self.recent_name));
                }
                Ok(value.unwrap())
            }
            Token::Sub => Ok(-self.prim(true)?),
            Token::LP => {
                let v = self.expr(true)?;
                if self.current_tok != Token::RP {
                    return Err(format!("')' Not Found: {}", self.msg));
                }
                // 吃掉一个元素
                self.get_token()?;
                Ok(v)
            }
            _ => Err(format!("unexpected({:?}): {}", self.current_tok, self.msg)),
        }

    }
    pub fn term(&mut self, get: bool) -> Result<f64, String> {
        let mut left = self.prim(get)?;
        loop {
            match self.current_tok {
                Token::Mul => {
                    left *= self.prim(true)?;
                    continue;
                }
                Token::Div => {
                    left /= self.prim(true)?;
                    continue;
                }
                _ => {
                    return Ok(left);
                }
            }
        }
    }
    pub fn expr(&mut self, get: bool) -> Result<f64, String> {
        let mut left = self.term(get)?;
        loop {
            match self.current_tok {
                Token::Add => {
                    left += self.term(true)?;
                    continue;
                }
                Token::Sub => {
                    left -= self.term(true)?;
                    continue;
                }     
                _ => {
                    return Ok(left);
                }    
            }
        }
    }
}

impl Default for Token {
    fn default() -> Token {
        Token::Print
    }
}

macro_rules! enum_create {
     ($($key: ident => $val: expr),+) => (
            #[allow(non_camel_case_types)]
            #[derive(Debug,Clone,PartialEq)]
            pub enum Token {
                Name,
                Number,
                End,
            $($key),+
            }
            impl Token {
                pub fn parse(c : &char)->Result<Self,()> {
                    match *c {
                        $($val => Ok(Token::$key)),+
                        , _=> Err(()),
                    }
                }
            }
    );
}

enum_create!(
Add=>'+',
Sub=> '-',
Mul=>'*',
Div=>'/',
Print=>';',
Assign=>'=',
LP=>'(',
RP=>')'
);
