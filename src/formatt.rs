use std::fs;

pub fn format_model_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let lines = parse_file_contents(&contents);
    let max_widths = calculate_max_widths(&lines);
    let formatted_output = generate_aligned_output(&lines, &max_widths);
    Ok(formatted_output)
}

fn parse_file_contents(contents: &str) -> Vec<Vec<&str>> {
    let mut lines = Vec::new();

    for line in contents.lines() {
        if !line.trim().starts_with('#') {
            let mut parts: Vec<&str> = line
                .trim()
                .split("  ")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .collect();

            if parts.len() > 1 && !parts[1].starts_with('[') {
                parts.insert(1, "");
            }

            lines.push(parts);
        }
    }

    lines
}

fn calculate_max_widths(lines: &[Vec<&str>]) -> Vec<usize> {
    let num_columns = lines.iter().map(|p| p.len()).max().unwrap_or(0);
    let mut max_widths = vec![0; num_columns];

    for column in 0..num_columns {
        let mut max_width = 0;
        for parts in lines {
            if let Some(part) = parts.get(column) {
                let len = part.len();
                if len > max_width {
                    max_width = len;
                }
            }
        }
        max_widths[column] = max_width;
    }

    max_widths
}

fn generate_aligned_output(lines: &[Vec<&str>], max_widths: &[usize]) -> String {
    let mut output = String::new();

    for parts in lines {
        for (i, part) in parts.iter().enumerate() {
            output.push_str(part);
            if i != parts.len() - 1 {
                let padding = {
                    let max_width = max_widths.get(i).unwrap();
                    let len = part.len();
                    " ".repeat(max_width.saturating_sub(len))
                };

                output.push_str(&padding);
                output.push_str("  ");
            }
        }

        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file_contents() {
        let contents = r#"# This is a comment
            a                 b  ->  c
            hello world  [x]  b  ->  c"#;
        let lines = parse_file_contents(contents);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].len(), 5);
        assert_eq!(lines[1].len(), 5);
        assert_eq!(lines[0][0], "a");
        assert_eq!(lines[0][1], "");
        assert_eq!(lines[0][2], "b");
        assert_eq!(lines[0][3], "->");
        assert_eq!(lines[0][4], "c");
        assert_eq!(lines[1][0], "hello world");
        assert_eq!(lines[1][1], "[x]");
        assert_eq!(lines[1][2], "b");
        assert_eq!(lines[1][3], "->");
        assert_eq!(lines[1][4], "c");
    }

    #[test]
    fn test_calculate_max_widths() {
        let lines = vec![
            vec!["a", "bb", "ccc"],
            vec!["dddd", "eee", "f"],
            vec!["ggggg", "hh", "i"],
        ];
        let max_widths = calculate_max_widths(&lines);
        assert_eq!(max_widths.len(), 3);
        assert_eq!(max_widths[0], 5);
        assert_eq!(max_widths[1], 3);
        assert_eq!(max_widths[2], 3);
    }

    #[test]
    fn test_generate_aligned_output() {
        let lines = vec![
            vec!["a", "bb", "ccc"],
            vec!["dddd", "eee", "f"],
            vec!["ggggg", "hh", "i"],
        ];
        let max_widths = vec![5, 3, 3];
        let output = generate_aligned_output(&lines, &max_widths);
        let expected_output = "a      bb   ccc\n\
                               dddd   eee  f\n\
                               ggggg  hh   i\n";
        assert_eq!(output, expected_output);
    }
}
