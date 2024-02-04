pub struct DownloadResult {
    pub name: String,
    pub description: &'static str,
    pub status: DownloadStatus,
}

#[derive(Eq, PartialEq)]
pub enum DownloadStatus {
    Downloaded,
    Failed,
    Skipped
}
