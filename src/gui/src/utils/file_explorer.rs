use crate::errors::GuiError;
use pdf::PdfGenerator;
use rfd::FileDialog;

pub trait FileExplorer: PdfGenerator {
    fn save(&self, name: &str) -> Result<(), GuiError> {
        let name = format!("{}.pdf", name);

        let file_path = FileDialog::new()
            .set_file_name(name)
            .add_filter("Documento PDF", &["pdf"])
            .save_file();

        if let Some(path) = file_path {
            self.save_as_pdf(&path)?;
        }

        Ok(())
    }
}
