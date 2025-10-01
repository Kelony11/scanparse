use std::collections::VecDeque;
use std::process;

// ===== TOKENS & SCANNER =====

#[derive(Debug)]
enum TOKEN {
    IDENTIFIER(String),
    NUMBER(String),
    PLUS,
    STAR,
    BOPEN,
    BCLOSE,
    ERROR(()),
    EOF,
}

struct SCANNER {
    index: usize,
    user_input: Vec<char>,
}

impl SCANNER {
    fn constructor(input_string: String) -> Self {
        SCANNER {
            index: 0,
            user_input: input_string.chars().collect(),
        }
    }

    fn look_up_current_char(&self) -> Option<char> {
        self.user_input.get(self.index).copied()
    }

    fn move_to_next_char(&mut self) -> Option<char> {
        let c = self.look_up_current_char();
        if c.is_some() {
            self.index += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.look_up_current_char(), Some(ch) if ch.is_whitespace()) {
            self.move_to_next_char();
        }
    }

    // small helper to accumulate while a predicate holds
    fn collect_while<F: Fn(char) -> bool>(&mut self, first: char, keep: F) -> String {
        let mut s = String::from(first);
        while let Some(next) = self.look_up_current_char() {
            if keep(next) {
                // safe unwrap: we just peeked it
                s.push(self.move_to_next_char().unwrap());
            } else {
                break;
            }
        }
        s
    }

    fn get_next_token(&mut self) -> Option<TOKEN> {
        self.skip_whitespace();
        let ch = self.move_to_next_char()?; 

        Some(match ch {
            '+' => TOKEN::PLUS,
            '*' => TOKEN::STAR,
            '(' => TOKEN::BOPEN,
            ')' => TOKEN::BCLOSE,
            d if d.is_ascii_digit() => {
                let num = self.collect_while(d, |c| c.is_ascii_digit());
                TOKEN::NUMBER(num)
            }
            a if a.is_alphabetic() => {
                let id = self.collect_while(a, |c| c.is_alphabetic());
                TOKEN::IDENTIFIER(id)
            }
            _ => TOKEN::ERROR(()),
        })
    }

    fn tokenize_the_line(&mut self) -> Vec<TOKEN> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.get_next_token() {
            tokens.push(tok);
        }
        tokens.push(TOKEN::EOF);
        tokens
    }
}

// ===== Minimal tree to control printed layout =====

#[derive(Clone)]
struct NODE {
    label: String,
    children: Vec<NODE>,
}

impl NODE {
    fn leaf(label: &str) -> NODE {
        NODE { label: label.to_string(), children: Vec::new() }
    }
    fn with(label: &str, children: Vec<NODE>) -> NODE {
        NODE { label: label.to_string(), children }
    }
}

// ===== PARSER =====

struct PARSER {
    index: usize,
    tokens: Vec<TOKEN>,
}

impl PARSER {
    fn constructor(tokens: Vec<TOKEN>) -> Self {
        PARSER { index: 0, tokens }
    }

    fn current_token(&self) -> &TOKEN {
        self.tokens.get(self.index).unwrap_or(&TOKEN::EOF)
    }

    fn move_to_next_token(&mut self) {
        if self.index < self.tokens.len() {
            self.index += 1;
        }
    }

    // EXPR -> TERM EXPRDASH
    fn parse_expr(&mut self) -> NODE {
        let t = self.parse_term();
        let d = self.parse_exprdash();
        NODE::with("EXPR", vec![t, d])
    }

    // EXPRDASH -> + TERM EXPRDASH | ε
    fn parse_exprdash(&mut self) -> NODE {
        if let TOKEN::PLUS = self.current_token() {
            self.move_to_next_token(); // '+'
            let rhs = self.parse_term();
            let more = self.parse_exprdash();
            NODE::with("EXPRDASH", vec![NODE::leaf("PLUS"), rhs, more])
        } else {
            NODE::with("EXPRDASH", vec![NODE::leaf("EPSILON")])
        }
    }

    // TERM -> FACTOR TERMDASH
    fn parse_term(&mut self) -> NODE {
        let f = self.parse_factor();
        let d = self.parse_termdash();
        NODE::with("TERM", vec![f, d])
    }

    // TERMDASH -> * FACTOR TERMDASH | ε
    fn parse_termdash(&mut self) -> NODE {
        if let TOKEN::STAR = self.current_token() {
            self.move_to_next_token(); // '*'
            let f = self.parse_factor();
            let more = self.parse_termdash();
            NODE::with("TERMDASH", vec![NODE::leaf("STAR"), f, more])
        } else {
            NODE::with("TERMDASH", vec![NODE::leaf("EPSILON")])
        }
    }

    // FACTOR -> IDENTIFIER | NUMBER | ( EXPR )
    fn parse_factor(&mut self) -> NODE {
        match self.current_token() {
            TOKEN::IDENTIFIER(name) => {
                let leaf = NODE::leaf(&format!("IDENTIFIER({})", name));
                self.move_to_next_token();
                NODE::with("FACTOR", vec![leaf])
            }
            TOKEN::NUMBER(n) => {
                let leaf = NODE::leaf(&format!("NUMBER({})", n));
                self.move_to_next_token();
                NODE::with("FACTOR", vec![leaf])
            }
            TOKEN::BOPEN => {
                self.move_to_next_token();
                let inside = self.parse_expr();
                if let TOKEN::BCLOSE = self.current_token() {
                    self.move_to_next_token();
                    NODE::with("FACTOR", vec![NODE::leaf("BOPEN"), inside, NODE::leaf("BCLOSE")])
                } else {
                    eprintln!("Error: missing closing parenthesis");
                    process::exit(1);
                }
            }
            other => {
                eprintln!("Error: Unexpected token in FACTOR: {:?}", other);
                process::exit(1);
            }
        }
    }
}

// ===== Breadth-first printer: one line per level =====

fn bfs_print(root: &NODE) {
    let mut q: VecDeque<(NODE, usize)> = VecDeque::new();
    q.push_back((root.clone(), 0));
    let mut level = 0usize;
    let mut line: Vec<String> = Vec::new();

    while let Some((node, lv)) = q.pop_front() {
        if lv != level {
            println!("{}", line.join(" "));
            line.clear();
            level = lv;
        }
        line.push(node.label.clone());
        for child in &node.children {
            q.push_back((child.clone(), lv + 1));
        }
    }

    if !line.is_empty() {
        println!("{}", line.join(" "));
    }
}

// ===== MAIN FUNCTION =====

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./scanparse <filename>");
        return;
    }

    let file_name = &args[1];
    let file_content = std::fs::read_to_string(file_name).expect("Failed to open file");

    for line in file_content.lines() {
        if line.trim().is_empty() {
            println!();
            continue;
        }
        let mut scanner = SCANNER::constructor(line.to_string());
        let tokens = scanner.tokenize_the_line();
        let mut parser = PARSER::constructor(tokens);

        let tree = parser.parse_expr();
        bfs_print(&tree);
        println!();
    }
}
