use time::{Date, OffsetDateTime};

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

        let mut years = target.year() - birth.year();
        let mut months = target.month() as i8 - birth.month() as i8;
        let mut days = target.day() as i8 - birth.day() as i8;

        if days < 0 {
            months -= 1;
            let prev_month_date = target
                .replace_day(1)
                .ok()
                .and_then(|d| d.previous_day())
                .unwrap_or(target);
            days +=
                time::util::days_in_month(prev_month_date.month(), prev_month_date.year()) as i8;
        }

        if months < 0 {
            years -= 1;
            months += 12;
        }

        Self {
            years: years as u8,
            months: months as u8,
            days: days as u8,
        }
    }
}

impl From<Date> for Age {
    fn from(birth: Date) -> Self {
        Self::between(birth, OffsetDateTime::now_utc().date())
    }
}

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.years, self.months, self.days)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn calc_age() {
        let birth = date!(2020 - 01 - 01);
        let target = date!(2024 - 05 - 05);
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 4);
        assert_eq!(age.months(), 4);
        assert_eq!(age.days(), 4);
    }

    #[test]
    fn test_birthday_not_reached_this_month() {
        let birth = date!(1990 - 05 - 15);
        let target = date!(2024 - 05 - 10);
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 33);
        assert_eq!(age.months(), 11);
        assert_eq!(age.days(), 25);
    }

    #[test]
    fn test_leap_year_february() {
        let birth = date!(2024 - 02 - 29);
        let target = date!(2025 - 02 - 28);
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 0);
        assert_eq!(age.months(), 11);
        assert_eq!(age.days(), 30);
    }

    #[test]
    fn test_target_before_birth() {
        let birth = date!(2024 - 01 - 01);
        let target = date!(2023 - 01 - 01);
        let age = Age::between(birth, target);

        assert_eq!(age.years(), 0);
        assert_eq!(age.months(), 0);
        assert_eq!(age.days(), 0);
    }

    #[test]
    fn test_from_now() {
        let birth = date!(2000 - 01 - 01);
        let age = Age::from(birth);
        assert!(age.years() >= 24);
    }
}
