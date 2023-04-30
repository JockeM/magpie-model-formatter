pub fn format_model_file(contents: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines = parse_file_contents(contents);
    let max_widths = calculate_max_widths(&lines);
    let formatted_output = generate_aligned_output(&lines, &max_widths);
    Ok(formatted_output)
}

#[derive(Clone, Debug, PartialEq)]
enum Line<'a> {
    Empty,
    Comment(&'a str),
    Model(Vec<&'a str>),
}

fn parse_file_contents(contents: &str) -> Vec<Line> {
    contents
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            if line.is_empty() {
                return Line::Empty;
            }
            if line.starts_with('#') {
                return Line::Comment(line);
            }

            let mut parts = line
                .split("  ")
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>();

            if parts.len() > 1 && !parts[1].starts_with('[') {
                parts.insert(1, "");
            }

            Line::Model(parts)
        })
        .collect()
}

fn calculate_max_widths(lines: &[Line]) -> Vec<usize> {
    let parts = lines
        .iter()
        .filter_map(|line| match line {
            Line::Empty => None,
            Line::Comment(_) => None,
            Line::Model(parts) => Some(parts),
        })
        .collect::<Vec<_>>();

    let num_columns = parts.iter().map(|x| x.len()).max().unwrap_or(0);
    (0..num_columns)
        .map(|column| {
            parts
                .iter()
                .flat_map(|parts| parts.get(column))
                .map(|part| part.len())
                .max()
                .unwrap_or(0)
        })
        .collect()
}

fn generate_aligned_output(lines: &[Line], max_widths: &[usize]) -> String {
    let mut output = String::new();

    for line in lines {
        match line {
            Line::Empty => (),
            Line::Comment(a) => output.push_str(a),
            Line::Model(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    output.push_str(part);

                    if i != parts.len() - 1 {
                        let max_width = max_widths.get(i).unwrap_or(&0);
                        let padding = " ".repeat(max_width.saturating_sub(part.len()));
                        output.push_str(&padding);
                        output.push_str("  ");
                    }
                }
            }
        };

        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_aligned_output() {
        let lines = vec![
            Line::Comment("# This is a comment"),
            Line::Empty,
            Line::Model(vec!["Model A", "[some description]", "->", "false"]),
            Line::Model(vec!["Model B", "12345678", "->", "true"]),
        ];

        let max_widths = calculate_max_widths(&lines);
        let expected_output = r#"# This is a comment

Model A  [some description]  ->  false
Model B  12345678            ->  true
"#;

        let actual_output = generate_aligned_output(&lines, &max_widths);

        assert_eq!(actual_output, expected_output);
    }
}