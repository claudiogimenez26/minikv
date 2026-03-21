pub fn parse_line(line: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        result.push(current);
                        current = String::new(); // ⚠️ sin clone
                    }
                }
                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            result.push(current);
        }

        result
    }