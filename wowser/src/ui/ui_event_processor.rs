use super::UiResult;

pub trait UiEventProcessor {
    fn process_events(&mut self) -> UiResult;
}
