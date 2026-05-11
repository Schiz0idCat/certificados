use crate::errors::GuiError;
use pdf::PdfGenerator;
use rfd::FileDialog;

pub struct FileExplorer;

impl FileExplorer {
    pub fn save<T: PdfGenerator>(name: &str, template: &T) -> Result<(), GuiError> {
        let name = format!("{}.pdf", name);

        let file_path = FileDialog::new()
            .set_file_name(name)
            .add_filter("Documento PDF", &["pdf"])
            .save_file();

        if let Some(path) = file_path {
            template.save_as_pdf(&path)?;
        }

        Ok(())
    }
}
