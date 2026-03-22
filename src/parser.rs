/// Parsea una línea de texto y devuelve sus partes separadas.
/// Soporta:
/// - Strings entre comillas (`"hola mundo"`)
/// - Caracteres escapados (`\"`, `\\`)
/// - Separación por espacios fuera de comillas
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
                    current = String::new();
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

#[cfg(test)]
mod parser_tests {
    use super::parse_line;

    #[test]
    fn test_parse_simple() {
        let result = parse_line("set clave valor");

        assert_eq!(result, vec!["set", "clave", "valor"]);
    }

    #[test]
    fn test_parse_con_comillas() {
        let result = parse_line(r#"set clave "valor con espacios""#);

        assert_eq!(result, vec!["set", "clave", "valor con espacios"]);
    }

    #[test]
    fn test_parse_escapes() {
        let result = parse_line(r#"set clave valor\ con\ espacios"#);

        assert_eq!(result, vec!["set", "clave", "valor con espacios"]);
    }
}

#[test]
fn test_parse_comillas_y_escape() {
    let result = parse_line(r#"set "hola \" mundo" "chau \" mundo""#);

    assert_eq!(result, vec!["set", "hola \" mundo", "chau \" mundo"]);
}
