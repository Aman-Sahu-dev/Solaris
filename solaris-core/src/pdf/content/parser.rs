use lopdf::{
    Document, Error, ObjectId,
    content::{Content, Operation},
};

pub fn page_operation(doc: &Document, page_id: ObjectId) -> Result<Vec<Operation>, Error> {
    let bytes = doc.get_page_content(page_id)?;
    Ok(Content::decode(&bytes)?.operations)
}
