#[derive(Debug, Clone)]
pub struct TextRun {
    pub font: String,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub raw: Vec<u8>,
    pub op_index: usize,
}
