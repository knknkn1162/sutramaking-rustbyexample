mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64)-> MathResult {
        if y == 0.0 { Err(MathError::DivisionByZero)} else {Ok(x/y)}
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 { Err(MathError::NegativeSquareRoot)} else { Ok(x.sqrt())}
    }

    fn ln(x: f64)-> MathResult {
        if x<= 0.0 { Err(MathError::NonPositiveLogarithm)} else {Ok(x.ln())}
    }

    pub fn op(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;
        sqrt(ln)

        // the trait bound `f64: std::ops::Try` is not satisfied
        //sqrt(ln)?

    }
}

pub fn test() {
    println!("{:?}", checked::op(1.0, 10.0));
}