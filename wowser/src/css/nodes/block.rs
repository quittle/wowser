use std::rc::Rc;

use super::{CssProperty, CssSelectorChain};

#[derive(PartialEq, Debug, Clone)]
pub struct CssBlock {
    pub selectors: Vec<CssSelectorChain>,
    pub properties: Vec<Rc<CssProperty>>,
}
