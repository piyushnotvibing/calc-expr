use crate::{errors::EvalError, parser::Expr};

pub fn eval(expr: &Expr) -> Result<f64, EvalError> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Add(l_expr, r_expr) => Ok(eval(l_expr)? + eval(r_expr)?),
        Expr::Sub(l_expr, r_expr_opt) => {
            if let Some(r_expr) = r_expr_opt {
                return Ok(eval(l_expr)? - eval(r_expr)?)
            }
            return Ok(-eval(l_expr)?)
        },
        Expr::Mul(l_expr, r_expr) => Ok(eval(l_expr)? * eval(r_expr)?),
        Expr::Div(l_expr, r_expr) => {
            let divisor = eval(r_expr)?;
            if divisor == 0.0 {
                return Err(EvalError::DivBy0);
            }
            Ok(eval(l_expr)? / divisor)
        }
        Expr::Pow(l_expr, r_expr) => {
            let base = eval(l_expr)?;
            let power = eval(r_expr)?;
            if base.is_sign_negative() && (power > 0.0 && power < 1.0) {
                return Err(EvalError::NaN);
            }
            Ok(base.powf(power))
        },
    }
}
