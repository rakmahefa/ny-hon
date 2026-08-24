mod browse;
mod history;
mod home;
mod library;
mod manga_detail;
mod updates;

pub use browse::{BrowsePage, BrowseState};
pub use history::{HistoryPage, HistoryFilter, HistoryState};
pub use home::HomePage;
pub use library::LibraryPage;
pub use manga_detail::{MangaDetailPage, MangaDetailState};
pub use updates::UpdatesPage;
