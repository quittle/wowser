use std::rc::Rc;

use super::{CssProperty, CssSelectorChainItem};

#[derive(PartialEq, Debug, Clone)]
pub struct CssBlock {
    pub selectors: Vec<Vec<CssSelectorChainItem>>,
    pub properties: Vec<Rc<CssProperty>>,
}
