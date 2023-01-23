use std::fs;
use std::path::Path;

fn find_scss_files(path: &Path) -> Vec<String> {
    let mut scss_files = Vec::new();

    if path.is_dir() {
        for file in fs::read_dir(&path).unwrap() {
            let file = file.unwrap();
            let file_path = file.path();

            if file_path.is_dir() {
                scss_files.extend(find_scss_files(&file_path));
            } else {
                if file_path.extension().map(|s| s == "scss").unwrap_or(false) {
                    scss_files.push(file_path.to_str().unwrap().to_owned())
                }
            }
        }
    }

    scss_files
}

fn main() {
    let project_path = Path::new("./");
    let scss_files = find_scss_files(project_path);
    for file in scss_files {
        println!("SASS CODE: {}", file);
    }

    let scss_code = "
        $main-color: #000000; 
        body {
            color: $main-color;
        }
    ";

    let lines: Vec<&str> = scss_code.lines().collect();
    let mut root_css = format!(":root {{\n");
    let mut new_scss_lines = Vec::new();

    for line in lines {
        let line = line.trim().replace("$", "--");

        if line.starts_with("--") {
            let formated_line = format!("{}\n", line);
            root_css += &formated_line;
        } else {
            if line.contains("--") {
                let css_rule = line.split("--").nth(0).unwrap();
                let css_variable_with_semicollon = line.split("--").nth(1).unwrap();
                let css_variable = css_variable_with_semicollon.replace(";", "");

                let formated_line = format!("{} var(--{});", css_rule, css_variable);
                new_scss_lines.push(formated_line);
            } else {
                new_scss_lines.push(line);
            }
        }
    }

    let new_scss_code = root_css + &new_scss_lines.join("\n");
}
