use lopdf::{Document, Error, ObjectId};
#[derive(Debug, Clone)]
pub struct PdfFrame {
    pub page_number: u32,
    pub media_box: [f64; 4],
    pub content_ids: Vec<ObjectId>,
}
const DEFAULT_MEDIA_BOX: [f64; 4] = [0.0, 0.0, 612.0, 792.0];
impl PdfFrame {
    pub fn extract_page_frame(
        doc: &Document,
        page_num: u32,
        page_id: ObjectId,
    ) -> Result<PdfFrame, Error> {
        let rec = doc.get_dictionary(page_id);
    }
}
