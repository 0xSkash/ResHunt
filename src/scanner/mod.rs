use std::collections::HashMap;
use strum::IntoEnumIterator;
use walkdir::DirEntry;
use crate::model::resource::{Resource, ResourceType};
use crate::scanner::color_scanner::ColorScanner;
use crate::scanner::image_scanner::ImageScanner;
use crate::scanner::string_scanner::StringScanner;

mod image_scanner;
mod string_scanner;
mod color_scanner;

pub fn collect_resources(folder_path: &str) -> HashMap<ResourceType, Vec<Resource>> {
    let mut resource_map : HashMap<ResourceType, Vec<Resource>> = HashMap::new();

    for resource_type in ResourceType::iter() {
        let resources = scan_for_resources(resource_type, folder_path);
        resource_map.insert(resource_type, resources);
    }

    return resource_map;
}

fn scan_for_resources(resource_type: ResourceType, folder_path: &str) -> Vec<Resource> {
    return match resource_type {
        ResourceType::String => StringScanner.scan(folder_path),
        ResourceType::Image => ImageScanner.scan(folder_path),
        ResourceType::Color => ColorScanner.scan(folder_path),
    }
}

pub trait ResourceScanner {
    fn scan(&self, folder_path: &str) -> Vec<Resource> {
        let directories_to_check = self.collect_directories(folder_path);

        for value in directories_to_check.iter() {
            println!("{:?}", value)
        }

        return self.format_entries(directories_to_check);
    }

    fn collect_directories(&self, folder_path: &str) -> Vec<DirEntry>;
    fn format_entries(&self, entries: Vec<DirEntry>) -> Vec<Resource>;
    fn resource_type(&self) -> ResourceType;
}