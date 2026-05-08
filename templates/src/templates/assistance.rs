use crate::errors::AssistanceError;

use age::Age;
use askama::Template;
use cfg::Cfg;
use pdf::PdfGenerator;
use rut::Rut;
use time::{Date, Time};

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
        start: Time,
        end: Time,
    ) -> Result<Self, AssistanceError> {
        if start > end {
            return Err(AssistanceError::Time);
        }

        let date_fmt = Cfg::global().date_fmt();
        let time_fmt = Cfg::global().time_fmt();
        let rut_fmt = Cfg::global().rut_fmt();

        Ok(Self {
            name: name.to_string(),
            birth: birth.format(date_fmt)?,
            age: Age::between(birth, today).to_string(),
            rut: rut.format(rut_fmt),
            today: today.format(date_fmt)?,
            appointment: appointment.format(date_fmt)?,
            start_time: start.format(time_fmt)?,
            end_time: end.format(time_fmt)?,
        })
    }
}

impl PdfGenerator for Assistance {}
