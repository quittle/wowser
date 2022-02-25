use super::{CssAtRule, CssBlock};

#[derive(PartialEq, Debug, Clone)]
pub enum CssTopLevelEntry {
    Block(CssBlock),
    AtRule(CssAtRule),
}
