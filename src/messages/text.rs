use super::WeChatMessage;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TextMessage {
    source: String,
    target: String,
    time: usize,
    id: usize,
    content: String,
}

impl WeChatMessage for TextMessage {
    fn source(&self) -> &str {
        &self.source
    }

    fn target(&self) -> &str {
        &self.target
    }

    fn time(&self) -> usize {
        self.time
    }

    fn id(&self) -> usize {
        self.id
    }

    fn from_xml<T>(xml: &str) -> T where T: WeChatMessage {
        unimplemented!()
    }
}

impl TextMessage {
    pub fn content(&self) -> String {
        self.content.clone()
    }
}
