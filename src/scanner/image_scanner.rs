use walkdir::DirEntry;
use crate::model::resource::{Resource, ResourceType};
use crate::scanner::ResourceScanner;

pub struct ImageScanner;

impl ResourceScanner for ImageScanner {
    //TODO: Also check Storyboard file
    fn collect_directories(&self, folder_path: &str) -> Vec<DirEntry> {
        walkdir::WalkDir::new(folder_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_dir() && entry.path().extension().unwrap_or_default() == self.resource_type().extension()
            })
            .collect()
    }

    fn format_entries(&self, entries: Vec<DirEntry>) -> Vec<Resource> {
        return vec![]
    }

    fn resource_type(&self) -> ResourceType {
        return ResourceType::Image;
    }
}