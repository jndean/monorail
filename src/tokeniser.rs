
use regex;


#[derive(Debug, Clone)]
pub struct Token {
    pub type_: String,
    pub string_: String,
    pub line: usize,
    pub col: usize
}


pub fn tokenise(data: &String) -> Vec<Token> {

    let name_regex = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9\.]*").unwrap();
    let number_regex = regex::Regex::new(r"^\d+(/\d+)?").unwrap();
    let ignore_regex = regex::Regex::new(r"^(([$][^$]*[$])|([ \t\r\n\f\v]+))").unwrap();
    let symbol_regex = regex::Regex::new(&(String::from(r"^(")
    + r"\+=|\-=|\*=|/="
    + r"|<=|>=|!=|=="
    + r"|:=|=:|=>"
    + r"|\+|\-|\*|/"
    + r"|=|<|>"
    + r"|\[|\]|\(|\)|\{|\}"
    + r"|;|~|#|,|&"
    + r")")).unwrap();

    let mut ret = Vec::new();
    let mut pos = 0;
    let mut line = 1;
    let mut col = 0;
    while pos < data.len() {
        
        if let Some(m) = name_regex.find(&data[pos..]) {
            let string_ = &data[pos .. pos + m.end()];
            ret.push(Token{
                type_: String::from("NAME"), 
                string_: String::from(string_),
                line, col
            });
            pos += m.end();
            col += m.end();
            continue;
        };

        if let Some(m) = symbol_regex.find(&data[pos..]) {
            let string_ = &data[pos .. pos + m.end()];
            ret.push(Token{
                type_: String::from("SYMBOL"), 
                string_: String::from(string_),
                line, col
            });
            pos += m.end();
            col += m.end();
            continue;
        };
        
        match number_regex.find(&data[pos..]) {
            Some(m) => {
                ret.push(Token{
                    type_: String::from("NUMBER"), 
                    string_: String::from(&data[pos .. pos + m.end()]),
                    line, col
                });
                pos += m.end();
                col += m.end();
                continue;
            }
            None => ()
        };

        match ignore_regex.find(&data[pos..]) {
            Some(m) => {
                pos += m.end();
                let mut matches: Vec<(usize, &str)> = 
                    data[m.start()..m.end()].match_indices(r"\n").collect();
                line += matches.len();
                if matches.len() > 0 {
                    let lastpos = matches.pop().unwrap().0;
                    col = m.end() - lastpos;
                } else {
                    col += m.end();
                }
                continue;
            }
            None => ()
        };

        println!("pos {}", pos);
        panic!("Unhandled input characters")
    }

    ret.push(Token {
        string_: String::from(""),
        type_: String::from("END_MARKER!"),
        line, col
    });
    ret
}