use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Local};

use crate::types::ExecutionId;

#[derive(Debug, Clone)]
pub struct Record {
    pub id: ExecutionId,
    pub start_time: DateTime<Local>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub end_time: DateTime<Local>,
    pub exit_code: i32,
    pub diff: Option<(u32, u32)>,
    pub previous_id: Option<ExecutionId>,
}

#[derive(Debug)]
struct StoreData {
    records: HashMap<ExecutionId, Record>,
    latest_id: Option<ExecutionId>,
}

#[derive(Clone, Debug)]
pub struct Store {
    data: Arc<RwLock<StoreData>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(StoreData {
                records: HashMap::new(),
                latest_id: None,
            })),
        }
    }

    pub fn add_record(&mut self, record: Record) {
        if let Ok(mut data) = self.data.write() {
            data.latest_id = Some(record.id);
            data.records.insert(record.id, record);
        }
    }

    pub fn get_record(&self, id: ExecutionId) -> Option<Record> {
        if let Ok(data) = self.data.read() {
            data.records.get(&id).cloned()
        } else {
            None
        }
    }

    pub fn get_latest_id(&self) -> Option<ExecutionId> {
        if let Ok(data) = self.data.read() {
            data.latest_id
        } else {
            None
        }
    }
}