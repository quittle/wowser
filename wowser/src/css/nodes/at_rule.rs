use super::CssBlock;

#[derive(PartialEq, Debug, Clone)]
pub struct CssAtRule {
    pub rule: String,
    pub args: Vec<String>,
    pub blocks: Vec<CssBlock>,
}
