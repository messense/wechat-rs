#[derive(Debug, Clone)]
pub struct QRCodeTicket {
    pub ticket: String,
    pub expire_seconds: u32,
    pub url: String,
}
