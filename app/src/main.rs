use pdf::PdfGenerator;
use rut::Rut;
use templates::Assistance;
use time::{
    OffsetDateTime,
    macros::{date, time},
};

use std::path::Path;
use std::str::FromStr;

fn main() {
    let name = "Agustín Guzmán";
    let birth = date!(2004 - 05 - 12);
    let rut = Rut::from_str("20099216-4").unwrap();
    let today = OffsetDateTime::now_utc().date();
    let appointment = OffsetDateTime::now_utc().date();
    let start = time!(14:00);
    let end = time!(15:00);

    let assistance = Assistance::try_new(name, birth, rut, today, appointment, start, end).unwrap();

    let path = Path::new("asistencia_agustin.pdf");

    match assistance.save_as_pdf(path) {
        Ok(_) => println!("¡PDF guardado exitosamente en {:?}!", path),
        Err(e) => eprintln!("Error al generar el PDF: {}", e),
    }
}
