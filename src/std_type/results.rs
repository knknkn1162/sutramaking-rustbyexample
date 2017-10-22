mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogalrithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64)-> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else { Ok(x/y)}
    }

    pub fn sqrt(x: f64)-> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else { Ok(x.sqrt())}
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogalrithm)
        } else { Ok(x.ln())}
    }

    pub fn op(x: f64, y: f64)-> MathResult {
        self::div(x,y)
            .and_then(self::ln)
            .and_then(self::sqrt)
    }
}

pub fn test() {
    let res = checked::op(1.0, 10.0);
    println!("res : {:?}",res.unwrap_err());
}