use crate::eval;
use crate::eval::env::Env;
use crate::read::parse;

pub(crate) fn eval(program: &str, env: &Env) -> Result<String, String> {
    let expression = parse(program).map_err(|e| e.to_string())?;
    let result = eval::eval(&expression, &env).map_err(|e| e.to_string())?;

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
            ("Nil", "Nil"),
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

    #[test]
    fn test_lambda() {
        let env = create_env();

        // Declare lambda
        eval("(def f (lambda (x y) (+ x y)))", &env).unwrap();

        let table = vec![
            ("f", Ok("(lambda (x y) (+ x y))")),
            ("(f 2 3)", Ok("5")),
            ("(f (f 2 6) 3)", Ok("11")),
            ("(f)", Err("Not enough values to fill-up all arguments")),
        ];

        for (program, expected_result) in table {
            let result = eval(program, &env);

            assert_eq!(
                result,
                expected_result
                    .map(|str| str.to_string())
                    .map_err(|str| str.to_string()),
                "{}",
                program,
            );
        }

        // Test varargs support
        assert_eq!(eval("((lambda (x . ys) x) 1 2 3 4)", &env).unwrap(), "1");
        assert_eq!(
            eval("((lambda (x . ys) ys) 1 2 3 4)", &env).unwrap(),
            "(2 3 4)"
        );
    }

    #[test]
    fn test_lambda_env_not_leaking() {
        let env = create_env();

        eval("(def f (lambda (x) (def a 10) a))", &env).unwrap();

        let result = eval("(f 10)", &env).unwrap();

        assert_eq!(env.get("x"), None);
        assert_eq!(env.get("a"), None);
        assert_eq!(result, "10")
    }

    #[test]
    fn test_def_expression() {
        let env = create_env();

        eval("(def x 10)", &env).unwrap();
        eval("(def y (list 10 20 30))", &env).unwrap();

        assert_eq!(eval("x", &env).unwrap(), "10");
        assert_eq!(eval("y", &env).unwrap(), "(10 20 30)");

        eval("(def (a b c) (list 10 20 30))", &env).unwrap();

        assert_eq!(eval("a", &env).unwrap(), "10");
        assert_eq!(eval("b", &env).unwrap(), "20");
        assert_eq!(eval("c", &env).unwrap(), "30");

        eval("(def (d . e) (list 10 20 30))", &env).unwrap();

        assert_eq!(eval("d", &env).unwrap(), "10");
        assert_eq!(eval("e", &env).unwrap(), "(20 30)");

        assert_eq!(
            eval("(def (d . e f) (list 10 20 30))", &env).err(),
            Some("Rest argument can be only one".to_string())
        );
        assert_eq!(
            eval("(def (d . e) 0)", &env).err(),
            Some("Unable to destruct non-list to symbols list: (d . e)".to_string())
        );
    }

    // #[test]
    fn test_cond_expression() {
        let env = create_env();

        let program = r#"
            (def fib-tail (lambda (n) 
              (def iter (lambda (n x y)
                (cond ((<= i 0) x) ((iter (dec i) y (+ x y))))))
              (iter n 0 1)))"#;

        eval(program, &env).unwrap();

        let result = eval("(fib-tail 5)", &env).unwrap();
    }
}
