use crate::eval::types::{Env, Expression, List, NIL};

struct StackEntry {
    env: Env,
    exp: List,
}

impl StackEntry {
    pub(crate) fn new(exp: List, env: Env) -> Self {
        Self { exp, env }
    }

    pub(crate) fn env(&self) -> &Env {
        &self.env
    }

    pub(crate) fn exp(&self) -> &List {
        &self.exp
    }
}

pub(crate) fn eval_list(list: List, env: Env) -> Expression {
    let mut stack = vec![];
    let mut result = vec![];

    stack.push(StackEntry::new(list, env));

    while !stack.is_empty() {
        let top_exp = stack.last().unwrap();

        match top_exp.exp() {
            Expression::List(list) => stack.push(StackEntry::new(list, env.clone())),
            _ => (),
        }
    }

    todo!()
}

// pub(crate) fn eval_iterative(exp: Expression, env: Env) -> Expression {
//     match exp {
//         Expression::Nil => Expression::Nil,
//         Expression::Int(n) => Expression::Int(n),
//         Expression::List(list) => {
//             let mut stack = vec![];
//             stack.push(StackEntry::new(list, env));
//             let mut eval_list = list;
//             let mut eval_result = List::new(Expression::Nil, None);
//
//             while eval_queue. {
//
//             }
//         }
//     }
//
//     Expression::Nil
// }
