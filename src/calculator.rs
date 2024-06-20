use std::str::FromStr;
use rust_decimal::{Decimal, Error};

pub struct Calculator {
    divend1: Decimal,
    divend2: Decimal,
}

impl Calculator {
    pub fn new(divend1: Decimal, divend2: Decimal) -> Self {
        Self {
            divend1,
            divend2,
        }
    }

    pub fn new_str(divend1: String, divend2: String) -> Result<Self, Error> {
        let div1 = Decimal::from_str(&divend1)?;
        let div2 = Decimal::from_str(&divend2)?;

        let calculator = Self {
            divend1: div1,
            divend2: div2,
        };
        Ok(calculator)
    }

    pub fn calc(&self) -> Decimal {
        self.divend1 / self.divend2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let divend1 = Decimal::new(10, 0); // 10
        let divend2 = Decimal::new(2, 0);  // 2
        let calculator = Calculator::new(divend1, divend2);

        assert_eq!(calculator.divend1, Decimal::new(10, 0));
        assert_eq!(calculator.divend2, Decimal::new(2, 0));
    }

    #[test]
    fn test_new_str() {
        let divend1 = "10".to_string();
        let divend2 = "2".to_string();
        let calculator = Calculator::new_str(divend1, divend2).unwrap();

        assert_eq!(calculator.divend1, Decimal::new(10, 0));
        assert_eq!(calculator.divend2, Decimal::new(2, 0));
    }

    #[test]
    fn test_calc() {
        let divend1 = Decimal::new(10, 0); // 10
        let divend2 = Decimal::new(2, 0);  // 2
        let calculator = Calculator::new(divend1, divend2);

        assert_eq!(calculator.calc(), Decimal::new(5, 0)); // 10 / 2 = 5
    }

    #[test]
    fn test_calc_with_new_str() {
        let divend1 = "10".to_string();
        let divend2 = "2".to_string();
        let calculator = Calculator::new_str(divend1, divend2).unwrap();

        assert_eq!(calculator.calc(), Decimal::new(5, 0)); // 10 / 2 = 5
    }

    #[test]
    fn test_new_str_invalid_input() {
        let divend1 = "10".to_string();
        let divend2 = "invalid".to_string();
        let result = Calculator::new_str(divend1, divend2);

        assert!(result.is_err());
    }
}
