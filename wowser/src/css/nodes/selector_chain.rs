use super::CssSelectorChainItem;

#[derive(PartialEq, Debug, Clone)]
pub struct CssSelectorChain {
    pub item: CssSelectorChainItem,
    pub next: Option<Box<CssSelectorChain>>,
}
