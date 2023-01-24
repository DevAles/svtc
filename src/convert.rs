use std::fs;
use std::path::Path;

pub struct SCSS {
    root: String,
    lines: Vec<String>,
}

impl SCSS {
    pub fn new() -> Self {
        SCSS {
            root: ":root {\n".to_string(),
            lines: Vec::new(),
        }
    }
}

pub fn file(path: &Path) {
    let scss_code = fs::read_to_string(&path).unwrap();

    if !scss_code.contains("$") {
        return;
    }

    let lines: Vec<&str> = scss_code.lines().collect();

    let mut new_scss = SCSS::new();

    for line in lines {
        let line = line.trim().replace("$", "--");

        if line.starts_with("--") {
            let formated_line = format!("{}\n", line);
            new_scss.root += &formated_line;
        } else {
            if line.contains("--") {
                let css_rule = line.split("--").nth(0).unwrap();
                let css_variable_with_semicollon = line.split("--").nth(1).unwrap();
                let css_variable = css_variable_with_semicollon.replace(";", "");

                let formated_line = format!("{} var(--{});", css_rule, css_variable);
                new_scss.lines.push(formated_line);
            } else {
                new_scss.lines.push(line);
            }
        }
    }

    new_scss.root += "}";

    let new_scss_code = new_scss.root + &new_scss.lines.join("\n");
    fs::write(path, new_scss_code).unwrap();
}

fn find_all_scss_files(path: &Path) -> Vec<String> {
    let mut scss_files = Vec::new();

    if path.is_dir() {
        for file in fs::read_dir(&path).unwrap() {
            let file = file.unwrap();
            let file_path = file.path();

            if file_path.is_dir() {
                scss_files.extend(find_all_scss_files(&file_path));
            } else {
                if file_path.extension().map(|s| s == "scss").unwrap_or(false) {
                    scss_files.push(file_path.to_str().unwrap().to_owned())
                }
            }
        }
    }

    scss_files
}

pub fn all() {
    let project_path = Path::new("./");
    let scss_paths = find_all_scss_files(project_path);

    for scss_path_str in scss_paths {
        let scss_path = Path::new(&scss_path_str);
        self::file(scss_path);
    }
}
