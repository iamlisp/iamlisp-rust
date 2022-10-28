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
    fn test_empty_program_eval_to_nil() {
        let env = create_env();

        assert_eq!("Nil", eval("", &env).unwrap())
    }

    #[test]
    fn test_empty_list_eval_to_empty_list() {
        let env = create_env();

        assert_eq!("()", eval("()", &env).unwrap())
    }

    #[test]
    fn test_list_constructor_eval_to_list() {
        let env = create_env();

        assert_eq!(
            r#"(1 2.5 "hello" true false)"#,
            eval(r#"(list 1 2.5 "hello" true false)"#, &env).unwrap()
        )
    }
}
