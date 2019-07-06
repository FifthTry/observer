use crate::context::{is_log_dir_exists, LOG_DIR};
use crate::{queue::Queue, utils};
use chrono::prelude::*;
use std::collections::HashMap;
use std::io::Write;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Frame {
    id: String,
    key: String,
    pub breadcrumbs: HashMap<String, serde_json::Value>,
    pub success: Option<bool>,
    pub result: Option<serde_json::Value>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub sub_frames: Vec<Frame>,
}

impl Frame {
    pub fn new(id: String) -> Frame {
        Frame {
            id,
            key: uuid::Uuid::new_v4().to_string(),
            breadcrumbs: HashMap::new(),
            success: None,
            result: None,
            start_time: Utc::now(),
            end_time: None,
            sub_frames: vec![],
        }
    }

    pub fn start(&mut self) -> &mut Self {
        self.start_time = Utc::now();
        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.end_time = Some(Utc::now());
        self
    }

    pub fn set_result(&mut self, result: serde_json::Value) -> &mut Self {
        self.result = Some(result);
        self
    }

    pub fn set_success(&mut self, is_success: bool) -> &mut Self {
        self.success = Some(is_success);
        self
    }

    pub fn add_sub_frame(&mut self, frame: Frame) {
        self.sub_frames.push(frame);
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn save(&self, critical: bool, queue: &Box<dyn Queue>) {
        if critical {
            self.enqueue(queue)
        } else {
            self.save_on_local()
        }
    }

    pub fn save_on_local(&self) {
        if is_log_dir_exists() {
            let path = LOG_DIR.to_owned() + self.id.as_str();
            if let Err(err) = utils::create_dir_if_not_exists(&path) {
                println!("Not able to create log_dir path: {}, {:?}", path, err);
                return;
            }
            match utils::create_file(&path, self.key.as_str()) {
                Ok(mut file) => {
                    if let Err(err) = file.write(json!(self).to_string().as_bytes()) {
                        println!("Frame write error {:#?}", err);
                    };
                }
                Err(err) => {
                    println!("Frame file create error {:#?}", err);
                }
            };
        }
    }

    pub fn enqueue(&self, queue: &Box<dyn Queue>) {
        queue.enqueue(json!(self))
    }

    //adding breadcrumbs
    pub fn add_breadcrumbs(&mut self, name: &str, value: serde_json::Value) {
        self.breadcrumbs.insert(name.to_string(), value);
    }
}
