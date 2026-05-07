use crate::DisplayOpt;
use crate::error::RutError;

#[derive(Debug, PartialEq)]
pub struct Rut {
    num: u32,
    dv: char,
}

impl Rut {
    pub fn num(&self) -> u32 {
        self.num
    }

    pub fn dv(&self) -> char {
        self.dv
    }

    fn is_valid(num: u32, dv: char) -> bool {
        let mut sum = 0;
        let mut multiplier = 2;
        let mut num_cp = num;

        while num_cp > 0 {
            let digit = num_cp % 10;

            sum += digit * multiplier;
            num_cp /= 10;

            multiplier += 1;

            if multiplier > 7 {
                multiplier = 2;
            }
        }

        let remainder = 11 - (sum % 11);

        let expected_dv = match remainder {
            11 => '0',
            10 => 'K',
            _ => (remainder as u8 + b'0') as char,
        };

        expected_dv == dv.to_ascii_uppercase()
    }

    pub fn try_new(num: u32, dv: char) -> Result<Self, RutError> {
        if !Self::is_valid(num, dv) {
            return Err(RutError::Mod11);
        }

        Ok(Self { num, dv })
    }

    pub fn format(&self, opt: DisplayOpt) -> String {
        let num = self.num.to_string();
        let num_len = num.len();
        let mut n = num_len + 1; // num + dv
        let dots = opt.dots();
        let hyphen = opt.hyphen();

        if dots && num_len > 3 {
            n += (num_len - 1) / 3;
        }

        if hyphen {
            n += 1;
        }

        let mut rut = String::with_capacity(n);

        if dots {
            for (i, c) in num.chars().enumerate() {
                let remaining = num_len - i;

                if i > 0 && remaining % 3 == 0 {
                    rut.push('.');
                }

                rut.push(c);
            }
        } else {
            rut.push_str(&num);
        }

        if opt.hyphen() {
            rut.push('-');
        }

        rut.push(self.dv);

        rut
    }
}

impl std::str::FromStr for Rut {
    type Err = RutError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean: String = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

        if clean.len() < 2 {
            return Err(RutError::TooShort);
        }

        let (num, dv) = clean.split_at(clean.len() - 1);

        let num: u32 = num.parse()?;

        let dv = dv
            .chars()
            .next()
            .ok_or(RutError::ParseDv)?
            .to_ascii_uppercase();

        Self::try_new(num, dv)
    }
}

impl std::fmt::Display for Rut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format(DisplayOpt::new(true, true)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn validate() {
        let validate_1 = Rut::is_valid(22715184, '6');
        let validate_2 = Rut::is_valid(32715184, 'k');

        assert_eq!(validate_1, true);
        assert_eq!(validate_2, false);
    }

    #[test]
    fn str() {
        assert_eq!(Rut::from_str("2").unwrap_err(), RutError::TooShort);

        assert_eq!(
            Rut::from_str("22.715.184-6").unwrap(),
            Rut::try_new(22715184, '6').unwrap()
        );

        let err = Rut::from_str("qw.ert.yui-k").unwrap_err();
        if let RutError::ParseBody(_) = err {
        } else {
            panic!("Expected ParseBody, got {:?}", err);
        }
    }

    #[test]
    fn display() {
        let rut = Rut::try_new(22715184, '6').unwrap();

        assert_eq!(rut.format(DisplayOpt::new(false, false)), "227151846");
        assert_eq!(rut.format(DisplayOpt::new(true, false)), "22.715.1846");
        assert_eq!(rut.format(DisplayOpt::new(false, true)), "22715184-6");
        assert_eq!(rut.format(DisplayOpt::new(true, true)), "22.715.184-6");
        assert_eq!(rut.to_string(), "22.715.184-6")
    }
}
