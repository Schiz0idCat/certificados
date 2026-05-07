use crate::errors::AssistanceError;

use age::Age;
use askama::Template;
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

        Ok(Self {
            name: name.to_string(),
            birth: birth.to_string(),
            age: Age::between(birth, today).to_string(),
            rut: rut.to_string(),
            today: today.to_string(),
            appointment: appointment.to_string(),
            start_time: start.to_string(),
            end_time: end.to_string(),
        })
    }
}

impl PdfGenerator for Assistance {}
