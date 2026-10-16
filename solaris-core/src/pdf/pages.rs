use lopdf::{Document, Error, Object, ObjectId};
#[derive(Debug, Clone)]
pub struct PdfFrame {
    pub page_number: u32,
    pub media_box: [f32; 4],
    pub content_ids: Vec<ObjectId>,
}
const DEFAULT_MEDIA_BOX: [f32; 4] = [0.0, 0.0, 612.0, 792.0];

fn parse_media_box(object: &Object) -> Option<[f32; 4]> {
    let arr = object.as_array().ok()?;
    if arr.len() != 4 {
        return None;
    }
    Some([
        arr[0].as_float().ok()?,
        arr[1].as_float().ok()?,
        arr[2].as_float().ok()?,
        arr[3].as_float().ok()?,
    ])
}
impl PdfFrame {
    pub fn extract_page_frame(
        doc: &Document,
        page_number: u32,
        page_id: ObjectId,
    ) -> Result<PdfFrame, Error> {
        let page_dict = doc.get_dictionary(page_id)?;
        let media_box = page_dict
            .get(b"MediaBox")
            .ok()
            .and_then(parse_media_box)
            .unwrap_or(DEFAULT_MEDIA_BOX);
        let content_ids = doc.get_page_contents(page_id);
        Ok(PdfFrame {
            page_number,
            media_box,
            content_ids,
        })
    }
}
