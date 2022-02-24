use super::CssBlock;

#[derive(PartialEq, Debug, Clone)]
pub struct CssDocument {
    pub blocks: Vec<CssBlock>,
}
