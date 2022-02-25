use super::CssTopLevelEntry;

#[derive(PartialEq, Debug, Clone)]
pub struct CssDocument {
    pub entries: Vec<CssTopLevelEntry>,
}
