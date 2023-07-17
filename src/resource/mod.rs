mod constants;

use self::constants::*;

use std::fs;

pub enum ResourceType {
    HTML,
    JS,
    CSS,
    ICO,
    XML,
    UNKNOWN,
}

impl ResourceType {
    pub fn new(filename: &str) -> ResourceType {
        match filename.split('.').last().unwrap() {
            FILE_TYPE_HTML => ResourceType::HTML,
            FILE_TYPE_JS => ResourceType::JS,
            FILE_TYPE_CSS => ResourceType::CSS,
            FILE_TYPE_ICO => ResourceType::ICO,
            FILE_TYPE_XML => ResourceType::XML,
            _ => ResourceType::UNKNOWN,
        }
    }

    pub fn get_mime_type(&self) -> String {
        String::from(match self {
            ResourceType::HTML => MIME_TYPE_HTML,
            ResourceType::JS => MIME_TYPE_JS,
            ResourceType::CSS => MIME_TYPE_CSS,
            ResourceType::ICO => MIME_TYPE_ICO,
            ResourceType::XML => MIME_TYPE_XML,
            ResourceType::UNKNOWN => MIME_TYPE_TEXT,
        })
    }
}

pub struct Resource {
    pub data: Vec<u8>,
    pub resource_type: ResourceType,
}

impl Resource {
    pub fn new(filename: &str) -> Option<Resource> {
        let mut res_path = RESOURCE_DIR_PATH.to_string();
        res_path.push_str(match filename {
            ROOT_PATH => INDEX_FILE,
            _ => filename,
        });

        match fs::read(&res_path) {
            Ok(file_content) => Some(Resource {
                data: file_content,
                resource_type: ResourceType::new(&res_path),
            }),
            Err(error) => {
                println!("For {} => {:?}", res_path, error);
                None
            }
        }
    }
}
