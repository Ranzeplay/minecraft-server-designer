use colored::Colorize;
use crate::downloader::universal_downloader::UniversalDownloadResult;

pub struct DownloadResult {
    pub name: String,
    pub description: &'static str,
    pub status: DownloadStatus,
}


#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum DownloadStatus {
    Downloaded,
    Failed,
    Skipped
}

impl DownloadResult {
    pub fn from_universal(res: &UniversalDownloadResult, name: String) -> DownloadResult {
        return DownloadResult {
            name,
            description: res.description,
            status: res.status.clone(),
        }
    }
    
    pub fn display_text(&self) {
        match self.status {
            DownloadStatus::Downloaded => println!("[Downloaded - {}]: {}", self.name.green(), self.description),
            DownloadStatus::Failed => println!("[Failed - {}]: {}", self.name.red(), self.description),
            DownloadStatus::Skipped => println!("[Skipped - {}]: {}", self.name.yellow(), self.description),
        }
    }
}
