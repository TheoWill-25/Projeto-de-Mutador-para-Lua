
#[derive(Debug)]
pub struct Candidate {
    pub line : usize,
    pub start: usize,
    pub end: usize,
    pub original: String,
    pub kind: String,
}

pub const SYMBOLS_LIST : [char; 4] = ['<', '>', '=', '~'];

pub fn start_end_finder(code: &str) -> (usize, usize) {
    let mut start_indx = 0;
    let mut end_indx = 0;
    for (i, line) in code.lines().enumerate(){
        if line.contains("--start mut") {
            start_indx  = i+1;
        }

        if line.contains("--end mut") {
            end_indx = i-1;
        }

    }
    if start_indx == 0  && end_indx == 0 {
        println!("N foi possivel achar a posição a ser alterada")
    }
    (start_indx, end_indx)
}

pub fn symbol_scanner(code: &str, start_indx : usize, end_indx: usize ) -> Vec<Candidate> {
    let mut symbols: Vec<Candidate> = Vec::new();
    let linhas: Vec<&str> = code.lines().collect();

    for i in start_indx .. end_indx+1 {
        let mut IF = false;
        let mut j = 0;
        let chars: Vec<char> = linhas[i].chars().collect();

        while j < chars.len() {
            let mut tam = 1;
            if chars[j] == 'i' && chars[j+1] == 'f' {
                IF = true;
                j += 2;
            }
            if SYMBOLS_LIST.contains((&chars[j])) {
                if chars[j+1] != ' '{
                    tam += 1;
                }
                symbols.push(Candidate { 
                             line: i+1, start: j, 
                             end: j + tam, 
                             original: (chars[j..j + tam].iter().collect::<String>()).to_string(), 
                             kind: "symbol".to_string() });
                j += 1;
            }

            j += 1;
        }
    }

    symbols
}


