use std::fs;
use std::fmt;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let path = args[1].to_string();
        let mut file = fs::read_to_string(path.as_str()).unwrap_or("NOFILE".to_string());
        if file == "NOFILE".to_string() {
            println!("Error: Cannot find '{}'", path);
            exit(1);
        }
        let file_type = detect_file_type(&path);
        let style = determine_style(file_type);
        let token = get_token(style);
        if token.len() > 2 {
            println!("Error: {}", token); 
            exit(1);                      
        }                                 
        fix_comments(&mut file, token);   
        let res = fs::write(path.as_str(), file);
        match res {
            Err(e) => println!("Error: {}", e),
            _ => println!("'{}' written successfully!", args[1]),
        }
    }
    else {
        println!("Usage: align-comments <filepath>")
    }
}

#[derive(Copy, Clone)]
enum Style {
    C,
    Python,
    Lua,
    Unknown,
}

fn style_to_string(style: Style) -> String {
    let result: String;
    match style {
        Style::C =>       result = "c".to_string(),
        Style::Python =>  result = "python".to_string(),
        Style::Lua =>     result = "lua".to_string(),
        Style::Unknown => result = "unknown".to_string(),
    }
    return result;
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", style_to_string(*self))
    }
}

fn get_token(file_type: Style) -> String {
    let token: String;
    match file_type {
        Style::C => token = "//".to_string(),
        Style::Python => token = "#".to_string(),
        Style::Lua => token = "--".to_string(),
        _ => token = "Unsupported file type".to_string(),
    }
    return token;
}

fn detect_file_type(path: &String) -> String {
    let idx = path.rfind(".");
    let result: String;
    match idx {
        Some(i) => result = path[i..].to_string(),
        None => result = "unknown".to_string(),
    }
    return result;
}

fn determine_style(ext: String) -> Style {
    let result: Style;
    match ext.as_str() {
        ".lua" => result = Style::Lua,
        ".py" | ".rb" | ".sh" => result = Style::Python,
        ".c" | ".cc" | ".cpp" => result = Style::C,
        ".js" | ".ts" => result = Style::C, 
        ".rs" => result = Style::C, 
        ".fs" | "fsi" | "cs" => result = Style::C, 
        ".swift" => result = Style::C, 
        ".scala" | ".sc" | ".kt" | ".java" => result = Style::C, 
        _ => result = Style::Unknown,
    }
    return result;
}

pub trait Utils<T> {
    fn at(&self, current: usize) -> Option<T>;
}

impl<T> Utils<T> for Vec<T>
where T: Clone {
    fn at(&self, idx: usize) -> Option<T> {
        if idx < self.len() {
            return Some(self[idx].clone());
        }
        None
    }
}

fn group_into_sections(comment_lines: Vec<usize>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::<Vec::<usize>>::new();
    let mut temp = Vec::<usize>::new();
    for (i, current) in comment_lines.iter().enumerate() {
        if i < comment_lines.len()-1 {
            let next = comment_lines.at(i+1).unwrap();
            if next - current > 1 {
                temp.push(current.clone());
                result.push(temp.clone());
                temp.clear();
            }
            else { temp.push(current.clone()); }
        }
        else { temp.push(current.clone()); } // last line 
    }
    if temp.len() > 0 { result.push(temp); }
    return result;
}

fn fix_comments(file: &mut String, token: String) {
    let lines: Vec<String> = file.lines().map(|x| x.to_string()).collect();
    let mut comment_lines = Vec::<usize>::new();
    for (i, line) in lines.iter().enumerate() {
        if i < lines.len()-1 {
            if line.contains(&token) {
                if lines.at(i+1).unwrap().contains(&token) {
                    comment_lines.push(i);
                    comment_lines.push(i+1);
                }
            }
        }
    }
    comment_lines.dedup();
    let grouped_sections = group_into_sections(comment_lines);
    for section in grouped_sections {
        fix_section(section, file, &token);
    }
}

fn fix_section(section: Vec<usize>, file: &mut String, token: &String) {
    let mut furthest = 0;
    let mut lines: Vec<String> = file.lines().map(|x| x.to_string()).collect();
    for num in section.clone() {
        let line = lines.at(num).unwrap();
        let idx = line.rfind(token).unwrap();
        if idx > furthest { furthest = idx; }
    }
    for i in section {
        let line = lines.at(i).unwrap();
        let idx = line.rfind(token).unwrap();
        let diff = furthest - idx;
        if diff > 0 { for _ in 0..diff { lines[i].insert(idx, ' '); } }
    }
    *file = lines.join("\n");
}