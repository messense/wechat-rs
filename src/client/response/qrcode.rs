#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QRCodeTicket {
    pub ticket: String,
    pub expire_seconds: u32,
    pub url: String,
}
