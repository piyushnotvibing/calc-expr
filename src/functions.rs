use std::collections::HashMap;

use crate::errors::EvalError;

pub trait Function {
    fn name(&self) -> &str;
    fn arity(&self) -> Option<usize>;
    fn call(&self, args: &[f64]) -> Result<f64, EvalError>;
}

pub struct Registry(pub HashMap<String, Box<dyn Function>>);
impl Registry {
    pub fn build() -> Self {
        let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();

        functions.insert(String::from("sin"), Box::new(TrigFunction::Sin));
        functions.insert(String::from("cos"), Box::new(TrigFunction::Cos));
        functions.insert(String::from("tan"), Box::new(TrigFunction::Tan));
        functions.insert(String::from("cosec"), Box::new(TrigFunction::Cosec));
        functions.insert(String::from("csc"), Box::new(TrigFunction::Cosec));
        functions.insert(String::from("sec"), Box::new(TrigFunction::Sec));
        functions.insert(String::from("cot"), Box::new(TrigFunction::Cot));
        functions.insert(String::from("sinh"), Box::new(TrigFunction::Sinh));
        functions.insert(String::from("cosh"), Box::new(TrigFunction::Cosh));
        functions.insert(String::from("tanh"), Box::new(TrigFunction::Tanh));

        functions.insert(String::from("log10"), Box::new(LogBase10));
        functions.insert(String::from("log2"), Box::new(LogBase2));
        functions.insert(String::from("ln"), Box::new(Ln));
        
        functions.insert(String::from("max"), Box::new(Max));
        functions.insert(String::from("min"), Box::new(Min));

        Self(functions)
    }
}

pub enum TrigFunction {
    Sin,
    Cos,
    Tan,
    Cosec,
    Sec,
    Cot,
    Sinh,
    Cosh,
    Tanh,
}

impl Function for TrigFunction {
    fn name(&self) -> &str {
        match self {
            TrigFunction::Sin => "sin",
            TrigFunction::Cos => "cos",
            TrigFunction::Tan => "tan",
            TrigFunction::Cosec => "cosec",
            TrigFunction::Sec => "sec",
            TrigFunction::Cot => "cot",
            TrigFunction::Sinh => "sinh",
            TrigFunction::Cosh => "cosh",
            TrigFunction::Tanh => "tanh",
        }
    }

    fn arity(&self) -> Option<usize> {
        Some(1)
    }

    fn call(&self, args: &[f64]) -> Result<f64, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::IncorrectNumOfArgs(self.name().to_string()));
        }

        if !args[0].is_finite() {
            return Err(EvalError::NaN);
        }

        match self {
            TrigFunction::Sin => Ok(args[0].sin()),
            TrigFunction::Cos => Ok(args[0].cos()),
            TrigFunction::Tan => Ok(args[0].tan()),
            TrigFunction::Cosec => Ok(1.0 / args[0].sin()),
            TrigFunction::Sec => Ok(1.0 / args[0].cos()),
            TrigFunction::Cot => Ok(1.0 / args[0].tan()),
            TrigFunction::Sinh => Ok(args[0].sinh()),
            TrigFunction::Cosh => Ok(args[0].cosh()),
            TrigFunction::Tanh => Ok(args[0].tanh()),
        }
    }
}

struct LogBase10;
impl Function for LogBase10 {
    fn name(&self) -> &str {
        "log10"
    }

    fn arity(&self) -> Option<usize> {
        Some(1)
    }

    fn call(&self, args: &[f64]) -> Result<f64, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::IncorrectNumOfArgs(self.name().to_string()));
        }
        if !args[0].is_finite() {
            return Err(EvalError::NaN);
        }

        let res = args[0].log10();
        if res.is_nan() || res.is_infinite() {
            return Err(EvalError::NaN);
        }
        Ok(res)
    }
}

struct LogBase2;
impl Function for LogBase2 {
    fn name(&self) -> &str {
        "log2"
    }

    fn arity(&self) -> Option<usize> {
        Some(1)
    }

    fn call(&self, args: &[f64]) -> Result<f64, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::IncorrectNumOfArgs(self.name().to_string()));
        }
        if !args[0].is_finite() {
            return Err(EvalError::NaN);
        }

        let res = args[0].log2();
        if res.is_nan() || res.is_infinite() {
            return Err(EvalError::NaN);
        }
        Ok(res)
    }
}

struct Ln;
impl Function for Ln {
    fn name(&self) -> &str {
        "ln"
    }

    fn arity(&self) -> Option<usize> {
        Some(1)
    }

    fn call(&self, args: &[f64]) -> Result<f64, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::IncorrectNumOfArgs(self.name().to_string()));
        }
        if !args[0].is_finite() {
            return Err(EvalError::NaN);
        }

        let res = args[0].ln();
        if res.is_nan() || res.is_infinite() {
            return Err(EvalError::NaN);
        }
        Ok(res)
    }
}

struct Max;
impl Function for Max {
    fn name(&self) -> &str {
        "max"
    }

    fn arity(&self) -> Option<usize> {
        None
    }

    fn call(&self, args: &[f64]) -> Result<f64, EvalError> {
        if args.len() == 0 {
            return Err(EvalError::IncorrectNumOfArgs(self.name().to_string()));
        }

        let mut max: f64 = f64::MIN;
        args.iter().for_each(|&f| {
            if f > max {
                max = f
            }
        });
        Ok(max)
    }
}

struct Min;
impl Function for Min {
    fn name(&self) -> &str {
        "min"
    }

    fn arity(&self) -> Option<usize> {
        None
    }

    fn call(&self, args: &[f64]) -> Result<f64, EvalError> {
        if args.len() == 0 {
            return Err(EvalError::IncorrectNumOfArgs(self.name().to_string()));
        }

        let mut min: f64 = f64::MAX;
        args.iter().for_each(|&f| {
            if f < min {
                min = f
            }
        });
        Ok(min)
    }
}
