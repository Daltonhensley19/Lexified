use std::io;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
enum Token {
    LPARM,
    RPARM,
    IDENT,
    SPACE,
    TAB,
    NEWLINE,
    PUNCT,
    EQUAL,
    NUM,
    EOL,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match *self {
            Self::LPARM => "(".to_string(),
            Self::RPARM => ")".to_string(),
            Self::IDENT => "IDENT".to_string(),
            Self::SPACE => " ".to_string(),
            Self::TAB => "\t".to_string(),
            Self::NEWLINE => "\n".to_string(),
            Self::PUNCT => ",".to_string(),
            Self::EQUAL => "=".to_string(),
            Self::NUM => "NUM".to_string(),
            Self::EOL => "EOL".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct Lexer {
    item: String,
    token_list: Vec<(Token, String)>,
}

macro_rules! token_push {
    ($self:ident, $ident:ident, $varient:ident) => {
        if $ident.len() != 0 {
            $self.token_list.push((Token::$varient, $ident.clone()));
            $ident.clear();
        }
    };
}

impl Lexer {
    fn tokenize(&mut self) {
        let mut identifer = String::new();
        let mut number = String::new();

        let split_txt = self.item.chars();
        for item in split_txt {
            //dbg!(item);
            match item {
                item if item.is_alphabetic() => identifer.push(item),
                item if item.is_numeric() => number.push(item),
                ')' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::RPARM, item.to_string()));
                }
                '(' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::LPARM, item.to_string()));
                }
                ' ' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::SPACE, item.to_string()));
                }
                '=' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::EQUAL, item.to_string()));
                }
                '\t' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::TAB, item.to_string()));
                }
                '\n' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::NEWLINE, item.to_string()));
                }
                ',' | '.' | ';' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::PUNCT, item.to_string()));
                }
                ':' | '?' | '!' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::PUNCT, item.to_string()));
                }
                '\'' => {
                    self.token_list.push((Token::PUNCT, item.to_string()));
                }
                '\0' => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    self.token_list.push((Token::EOL, item.to_string()));
                }
                _ => {
                    token_push!(self, identifer, IDENT);
                    token_push!(self, number, NUM);
                    println!("(Unknown, {})", item);
                }
            }
        }
        token_push!(self, identifer, IDENT);
        token_push!(self, number, NUM);
    }
}

fn main() {
    let mut test_string = "".to_owned();

    println!("Enter a string for the lexer to tokenize: ");
    io::stdin()
        .read_line(&mut test_string)
        .expect("Unable to get input");

    let mut lex = Lexer {
        item: test_string,
        token_list: Default::default(),
    };

    lex.tokenize();
    for item in lex.token_list.iter() {
        println!("{:?}", item);
    }
}
