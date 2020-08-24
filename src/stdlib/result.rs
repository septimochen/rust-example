mod checked {

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    #[allow(dead_code)]
    pub type MathResult = Result<f64, MathError>;

    #[allow(dead_code)]
    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    #[allow(dead_code)]
    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    #[allow(dead_code)]
    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

#[allow(dead_code)]
fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(_why) => panic!("{:?}", _why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(_why) => panic!("{:?}", _why),
            Ok(log) => match checked::sqrt(log) {
                Err(_why) => panic!("{:?}", _why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

#[test]
fn result_test() {
    let r1 = op(10.0, 1.0);
    println!("{}", r1);
    assert_eq!(r1, 10.0_f64.ln().sqrt());
}
