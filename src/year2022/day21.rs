#[cfg(test)]
mod day21 {
    use std::collections::HashMap;

    use anyhow::Result;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day21::{get_data, Expression, Operator},
    };

    enum Frame<'a> {
        EnsureValueFor(&'a String),
        FinishComputingValueFor(&'a String),
    }

    fn compute(root: String, data: &HashMap<String, Expression>) -> i64 {
        use Expression::*;
        use Frame::*;
        use Operator::*;
        let mut results: HashMap<String, i64> = HashMap::new();
        let mut stack = vec![EnsureValueFor(&root)];

        while let Some(frame) = stack.pop() {
            match frame {
                EnsureValueFor(name) => {
                    let expr = &data[name];
                    match expr {
                        Num(n) => {
                            results.insert(name.clone(), *n);
                        }
                        Operation(l, _, r) => {
                            stack.push(FinishComputingValueFor(name));
                            stack.push(EnsureValueFor(r));
                            stack.push(EnsureValueFor(l));
                        }
                    }
                }
                FinishComputingValueFor(name) => {
                    let expr = &data[name];
                    match expr {
                        Num(_) => {}
                        Operation(l, op, r) => {
                            let nl = results[l];
                            let nr = results[r];
                            let result = match op {
                                Add => nl + nr,
                                Sub => nl - nr,
                                Mul => nl * nr,
                                Div => nl / nr,
                            };
                            results.insert(name.clone(), result);
                        }
                    }
                }
            }
        }

        results[&root]
    }

    fn a(data: Vec<(String, Expression)>) -> i64 {
        let data: HashMap<String, Expression> = data.into_iter().collect();
        compute(String::from("root"), &data)
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 152);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 70674280581468);
        Ok(())
    }

    #[derive(PartialEq)]
    enum Ast {
        Ref(String),
        Operation(String, Box<Ast>, Operator, Box<Ast>),
    }

    impl Ast {
        fn contains_humn(&self) -> bool {
            match self {
                Ast::Ref(name) => name == "humn",
                Ast::Operation(_, l, _, r) => l.contains_humn() || r.contains_humn(),
            }
        }

        fn operation(name: String, left: Box<Ast>, op: Operator, right: Box<Ast>) -> Box<Ast> {
            Box::new(Ast::Operation(name, left, op, right))
        }

        fn compute(&self, data: &HashMap<String, Expression>) -> i64 {
            match self {
                Ast::Ref(n) => match data[n] {
                    Expression::Num(n) => n,
                    _ => unreachable!(),
                },
                Ast::Operation(n, _, _, _) => compute(n.clone(), data),
            }
        }
    }

    fn build_ast(name: &String, data: &HashMap<String, Expression>) -> Box<Ast> {
        use Expression::*;

        if name == "humn" {
            Box::new(Ast::Ref(String::from("humn")));
        }

        let expr = &data[name];
        match expr {
            Num(_) => Box::new(Ast::Ref(name.clone())),
            Operation(l, op, r) => Box::new(Ast::Operation(
                name.clone(),
                build_ast(l, data),
                *op,
                build_ast(r, data),
            )),
        }
    }

    fn b(data: Vec<(String, Expression)>) -> i64 {
        use Expression::*;
        use Operator::*;
        let root = String::from("root");
        let data: HashMap<String, Expression> = data.into_iter().collect();
        let root_expr = &data[&root];
        let (left, right) = match root_expr {
            Operation(l, _, r) => (l, r),
            _ => unreachable!(),
        };

        let mut left_ast = build_ast(left, &data);
        let mut right_ast = build_ast(right, &data);

        if right_ast.contains_humn() {
            (left_ast, right_ast) = (right_ast, left_ast);
        }

        let mut right_val = right_ast.compute(&data);

        while *left_ast != Ast::Ref(String::from("humn")) {
            match *left_ast {
                Ast::Operation(_, l, op, r) => match op {
                    Add => {
                        if l.contains_humn() {
                            left_ast = l;
                            right_val -= r.compute(&data);
                        } else {
                            left_ast = r;
                            right_val -= l.compute(&data);
                        }
                    }
                    Sub => {
                        if l.contains_humn() {
                            left_ast = l;
                            right_val += r.compute(&data);
                        } else {
                            left_ast = r;
                            right_val = l.compute(&data) - right_val;
                        }
                    }
                    Mul => {
                        if l.contains_humn() {
                            left_ast = l;
                            right_val /= r.compute(&data);
                        } else {
                            left_ast = r;
                            right_val /= l.compute(&data);
                        }
                    }
                    Div => {
                        if l.contains_humn() {
                            left_ast = l;
                            right_val *= r.compute(&data);
                        } else {
                            left_ast = r;
                            right_val = l.compute(&data) / right_val;
                        }
                    }
                },
                Ast::Ref(_) => unreachable!(),
            }
        }

        right_val
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 301);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 3243420789721);
        Ok(())
    }
}
