use lopdf::Document;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum PdfDocumentError {
    #[error("failed to load the file")]
    Load {
        path: PathBuf,
        #[source]
        source: lopdf::Error,
    },
}
#[derive(Debug)]
pub struct PdfDocument {
    pdf: Document,
}
impl PdfDocument {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, PdfDocumentError> {
        let inner: Document =
            Document::load(path.as_ref()).map_err(|source| PdfDocumentError::Load {
                path: path.as_ref().to_path_buf(),
                source,
            })?;
        Ok(Self { pdf: inner })
    }
}
