use crate::{errors::EvalError, functions::Registry, parser::Expr};

pub fn eval(expr: &Expr, registry: &Registry) -> Result<f64, EvalError> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Add(l_expr, r_expr) => Ok(eval(l_expr, registry)? + eval(r_expr, registry)?),
        Expr::Sub(l_expr, r_expr_opt) => {
            if let Some(r_expr) = r_expr_opt {
                return Ok(eval(l_expr, registry)? - eval(r_expr, registry)?);
            }
            return Ok(-eval(l_expr, registry)?);
        }
        Expr::Mul(l_expr, r_expr) => Ok(eval(l_expr, registry)? * eval(r_expr, registry)?),
        Expr::Div(l_expr, r_expr) => {
            let divisor = eval(r_expr, registry)?;
            if divisor == 0.0 {
                return Err(EvalError::DivBy0);
            }
            Ok(eval(l_expr, registry)? / divisor)
        }
        Expr::Pow(l_expr, r_expr) => {
            let base = eval(l_expr, registry)?;
            let power = eval(r_expr, registry)?;
            let res = base.powf(power);
            if res.is_nan() || res.is_infinite() {
                return Err(EvalError::NaN)
            }
            Ok(res)
        }
        Expr::Call(func, args) => {
            if let Some(f) = registry.0.get(func.as_str()) {
                if let Some(arity) = f.arity() {
                    if args.len() != arity {
                        return Err(EvalError::IncorrectNumOfArgs(func.clone()));
                    }
                    let args = args
                        .into_iter()
                        .map(|expr| eval(expr, registry))
                        .collect::<Result<Vec<f64>, EvalError>>()?;

                    f.call(&args)
                } else {
                    let args = args
                        .into_iter()
                        .map(|expr| eval(expr, registry))
                        .collect::<Result<Vec<f64>, EvalError>>()?;

                    f.call(&args)
                }
            } else {
                Err(EvalError::InvalidFunction(func.clone()))
            }
        }
    }
}
