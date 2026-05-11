use jiff::Unit;
use jiff::Zoned;
use jiff::civil::Date;

pub struct Age {
    years: u8,
    months: u8,
    days: u8,
}

impl Age {
    pub fn years(&self) -> u8 {
        self.years
    }

    pub fn months(&self) -> u8 {
        self.months
    }

    pub fn days(&self) -> u8 {
        self.days
    }

    pub fn between(birth: Date, target: Date) -> Self {
        if target < birth {
            return Self {
                years: 0,
                months: 0,
                days: 0,
            };
        }

        let span = birth.until((Unit::Year, target)).unwrap();

        Self {
            years: span.get_years() as u8,
            months: span.get_months() as u8,
            days: span.get_days() as u8,
        }
    }
}

impl From<Date> for Age {
    fn from(birth: Date) -> Self {
        Self::between(birth, Zoned::now().date())
    }
}

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format_age = |value: u8, singular: &str, plural: &str| -> String {
            match value {
                0 => String::new(),
                1 => format!("1 {}", singular),
                _ => format!("{} {}", value, plural),
            }
        };

        let years = format_age(self.years, "Año", "Años");
        let months = format_age(self.months, "Mes", "Meses");
        let days = format_age(self.days, "Día", "Días");

        let parts: Vec<&str> = [years.as_str(), months.as_str(), days.as_str()]
            .iter()
            .filter(|s| !s.is_empty())
            .copied()
            .collect();

        write!(f, "{}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_age() {
        let birth = Date::new(2020, 1, 1).unwrap();
        let target = Date::new(2024, 5, 5).unwrap();
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 4);
        assert_eq!(age.months(), 4);
        assert_eq!(age.days(), 4);
    }

    #[test]
    fn test_birthday_not_reached_this_month() {
        let birth = Date::new(1990, 5, 15).unwrap();
        let target = Date::new(2024, 5, 10).unwrap();
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 33);
        assert_eq!(age.months(), 11);
        assert_eq!(age.days(), 25);
    }

    #[test]
    fn test_leap_year_february() {
        let birth = Date::new(2024, 2, 29).unwrap();
        let target = Date::new(2025, 2, 28).unwrap();
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 0);
        assert_eq!(age.months(), 11);
        assert_eq!(age.days(), 30);
    }

    #[test]
    fn test_target_before_birth() {
        let birth = Date::new(2024, 1, 1).unwrap();
        let target = Date::new(2023, 1, 1).unwrap();
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 0);
        assert_eq!(age.months(), 0);
        assert_eq!(age.days(), 0);
    }

    #[test]
    fn test_from_now() {
        let birth = Date::new(2000, 1, 1).unwrap();
        let age = Age::from(birth);
        assert!(age.years() >= 24);
    }
}
