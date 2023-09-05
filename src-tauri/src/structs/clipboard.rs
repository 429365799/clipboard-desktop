use std::fs;

use clipboardrs::api::{ClipboardFile, ClipboardData};
use serde::{Serialize, ser::SerializeStruct};
use tauri::api::path::cache_dir;
use uuid::Uuid;

pub(crate) struct MyFile(ClipboardFile);

impl Serialize for MyFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("MyFile", 2)?;
        s.serialize_field("path", &self.0.path)?;
        s.serialize_field("size", &self.0.size)?;
        s.end()
    }
}

pub(crate) struct MyClipboardData {
    key: String,
    len: usize,
    text: Option<String>,
    html: Option<String>,
    image: Option<String>,
    files: Option<Vec<MyFile>>,
}

impl MyClipboardData {
    pub(crate) fn new(raw_data: ClipboardData) -> MyClipboardData {
        let mut len: usize = 0;
        let mut data = MyClipboardData {
            key: Uuid::new_v4().to_string(),
            len: 0,
            text: None,
            html: None,
            image: None,
            files: None,
        };

        if let Some(val) = raw_data.text {
            len += 1;
            data.text = Some(val);
        }
        if let Some(val) = raw_data.html {
            len += 1;
            data.html = Some(val);
        }
        if let Some(val) = raw_data.image {
            if let Some(p) = cache_dir() {
                let parent_dir = p.join("clipboard-tauri");
                let path = parent_dir.join(format!("{}.png", Uuid::new_v4()));
                if let Ok(()) = fs::create_dir_all(parent_dir) {
                    if let Some(path) = path.to_str() {
                        if let Ok(()) = val.save(path) {
                            len += 1;
                            data.image = Some(path.to_string());
                        }
                    }
                }
            }
        }
        if let Some(val) = raw_data.files {
            let mut files = vec![];
            for f in val {
                files.push(MyFile(f));
            }
            data.files = Some(files);
        }

        data.len = len;

        data
    }
}

impl Serialize for MyClipboardData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("MyClipboardData", self.len)?;
        if let Some(text) = &self.text {
            s.serialize_field("text", text)?;
        }
        if let Some(html) = &self.html {
            s.serialize_field("html", html)?;
        }
        if let Some(img) = &self.image {
            s.serialize_field("image", img)?;
        }
        if let Some(files) = &self.files {
            s.serialize_field("files", files)?;
        }
        s.end()
    }
}
