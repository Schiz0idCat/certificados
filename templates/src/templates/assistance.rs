use crate::errors::AssistanceError;

use age::Age;
use askama::Template;
use cfg::Cfg;
use jiff::civil::Date;
use jiff::fmt::strtime;
use pdf::PdfGenerator;
use rut::Rut;
use time_utils::TimeRange;

#[derive(Template)]
#[template(path = "assistance.html")]
pub struct Assistance {
    name: String,
    birth: String,
    age: String,
    rut: String,
    today: String,
    appointment: String,
    start_time: String,
    end_time: String,
}

impl Assistance {
    pub fn try_new(
        name: impl ToString,
        birth: Date,
        rut: Rut,
        today: Date,
        appointment: Date,
        range: TimeRange,
    ) -> Result<Self, AssistanceError> {
        if birth >= appointment {
            return Err(AssistanceError::BirthAfterAppointment);
        }

        if birth >= today {
            return Err(AssistanceError::BirthAfterToday);
        }

        let date_fmt = Cfg::global().date_fmt();
        let time_fmt = Cfg::global().time_fmt();
        let rut_fmt = Cfg::global().rut_fmt();

        Ok(Self {
            name: to_title_case(name),
            birth: strtime::format(date_fmt, birth)?,
            age: Age::between(birth, today).to_string(),
            rut: rut.format(rut_fmt),
            today: strtime::format(date_fmt, today)?,
            appointment: strtime::format(date_fmt, appointment)?,
            start_time: strtime::format(time_fmt, range.start())?,
            end_time: strtime::format(time_fmt, range.end())?,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn today(&self) -> &str {
        &self.today
    }
}

fn to_title_case(str: impl ToString) -> String {
    str.to_string()
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

impl PdfGenerator for Assistance {}
