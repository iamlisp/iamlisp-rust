use crate::eval;
use crate::eval::env::Env;
use crate::read::parse;

pub(crate) fn eval(program: &str, env: &Env) -> anyhow::Result<String> {
    let expression = parse(program)?;
    let result = eval::eval(&expression, &env)?;
    Ok(format!("{}", result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::create_env;

    #[test]
    fn test_primitives() {
        let table = vec![
            ("1", "1"),
            ("1.5", "1.5"),
            (r#""string""#, r#""string""#),
            (r#""string\"string""#, r#""string"string""#),
            ("true", "true"),
            ("false", "false"),
            ("", "Nil"),
        ];
        let env = create_env();

        for (program, expected_result) in table {
            let result = eval(program, &env).unwrap();

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_list_constructor() {
        let table = vec![
            ("()", "()"),
            ("(list)", "()"),
            (
                r#"(list 1 2.5 "hello" true false)"#,
                r#"(1 2.5 "hello" true false)"#,
            ),
        ];
        let env = create_env();

        for (program, expected_result) in table {
            let result = eval(program, &env).unwrap();

            assert_eq!(result, expected_result);
        }
    }
}
