use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ResourceType {
    String,
    Image,
    // Maybe???
    Color,
}

impl ResourceType {
    pub fn extension(&self) -> &'static str {
        match self {
            ResourceType::String => "strings",
            ResourceType::Image => "imageset",
            ResourceType::Color => "colorset",
        }
    }
}

#[derive(Debug)]
pub struct Resource {
    pub path: String,
    pub resource_type: ResourceType,
    pub original_key: String,
    pub formatted_key: String,
}

impl Resource {
    pub fn new(
        path: &str,
        resource_type: ResourceType,
        original_key: &str,
        formatted_key: &str,
    ) -> Resource {
        Resource {
            path: path.to_string(),
            resource_type,
            original_key: original_key.to_string(),
            formatted_key: formatted_key.to_string(),
        }
    }
}