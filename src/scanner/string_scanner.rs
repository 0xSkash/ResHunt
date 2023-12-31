use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use walkdir::DirEntry;

use crate::model::resource::{Resource, ResourceType};
use crate::scanner::ResourceScanner;

// This needs to be ignored, generated by the R.swift library
const GENERATED_RESOURCE_FILE_NAME: &str = "R.generated.swift";

const STRING_RESOURCE_PREFIX: &str = "R.string.";

pub struct StringScanner;

impl StringScanner {
    fn collect_string_keys(dir: &DirEntry) -> Vec<String> {
        let key_pattern = Regex::new(r#""([^"]+)"\s*=\s*"#).unwrap();
        return match File::open(dir.path()) {
            Ok(file) => {
                let buffer = BufReader::new(file);

                buffer.lines()
                    .filter_map(Result::ok)
                    .filter_map(|line| {
                        key_pattern.captures(&line).and_then(|captures| captures.get(1).map(|m| m.as_str().to_string()))
                    })
                    .collect()
            }
            Err(..) => {
                vec![]
            }
        };
    }

    fn format_key(key: &str, origin: &str) -> String {
        let mut formatted_key = format!("{}{}.", STRING_RESOURCE_PREFIX, origin);
        let mut should_uppercase = false;

        for c in key.chars() {
            if should_uppercase {
                formatted_key.push(c.to_uppercase().next().unwrap());
                should_uppercase = false;
            } else {
                if c == '.' {
                    should_uppercase = true;
                } else {
                    formatted_key.push(c);
                }
            }
        }

        formatted_key
    }

    fn extract_origin_name(&self, file_name: &str) -> String {
        let extension_to_remove = format!(".{}", self.resource_type().extension());
        return file_name.replace(&extension_to_remove, "");
    }
}

impl ResourceScanner for StringScanner {
    fn collect_directories(&self, folder_path: &str) -> Vec<DirEntry> {
        walkdir::WalkDir::new(folder_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension().unwrap_or_default() == self.resource_type().extension()
            })
            .filter(|entry| {
                entry.path().file_name().unwrap_or_default() != GENERATED_RESOURCE_FILE_NAME
            })
            .collect()
    }

    fn format_entries(&self, entries: Vec<DirEntry>) -> Vec<Resource> {
        entries
            .iter()
            .flat_map(|entry| {
                let resource_path = entry.path().to_str().unwrap_or_default();
                let origin_name = entry.file_name().to_str().unwrap_or_default();
                let keys = StringScanner::collect_string_keys(entry);

                keys.iter().map(|key| {
                    let origin_name = self.extract_origin_name(origin_name).to_lowercase();
                    Resource::new(
                        resource_path,
                        ResourceType::String,
                        key,
                        &StringScanner::format_key(key, &origin_name),
                    )
                }).collect::<Vec<Resource>>()
            })
            .collect()
    }

    fn resource_type(&self) -> ResourceType {
        return ResourceType::String;
    }
}