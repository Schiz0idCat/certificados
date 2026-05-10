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
        let date_fmt = Cfg::global().date_fmt();
        let time_fmt = Cfg::global().time_fmt();
        let rut_fmt = Cfg::global().rut_fmt();

        Ok(Self {
            name: name.to_string(),
            birth: strtime::format(date_fmt, birth)?,
            age: Age::between(birth, today).to_string(),
            rut: rut.format(rut_fmt),
            today: strtime::format(date_fmt, today)?,
            appointment: strtime::format(date_fmt, appointment)?,
            start_time: strtime::format(time_fmt, range.start())?,
            end_time: strtime::format(time_fmt, range.end())?,
        })
    }
}

impl PdfGenerator for Assistance {}
