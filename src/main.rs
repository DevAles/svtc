fn main() {
    let sass_code = "
        $main-color: #000000; 
        body {
            color: $main-color;
        }
    ";

    let lines: Vec<&str> = sass_code.lines().collect();
    let mut root_css = format!(":root {{\n");
    let mut new_sass_lines = Vec::new();

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
                new_sass_lines.push(formated_line);
            } else {
                new_sass_lines.push(line);
            }
        }
    }

    let new_sass_code = root_css + &new_sass_lines.join("\n");
}
