#[derive(PartialEq, Debug, Clone)]
pub enum CssSelectorChainItem {
    Tag(String),
    Class(String),
    Id(String),
}
