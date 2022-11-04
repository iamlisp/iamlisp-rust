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

    #[test]
    fn test_math() {
        let table = vec![
            ("(+ 1 2)", "3"),
            ("(+ 2.5 3.5)", "6"),
            ("(- 10 6)", "4"),
            ("(- 10.5 3.5)", "7"),
            ("(* 2 3)", "6"),
            ("(* 2.5 3.5)", "8.75"),
            ("(/ 10 2)", "5"),
            ("(/ 10.0 4.0)", "2.5"),
        ];
        let env = create_env();

        for (program, expected_result) in table {
            let result = eval(program, &env).unwrap();

            assert_eq!(result, expected_result);
        }
    }
}
