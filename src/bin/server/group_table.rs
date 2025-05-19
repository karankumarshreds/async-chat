#![allow(unused)]
use crate::group::Group;
use std::{collections::HashMap, sync::{Mutex, Arc}};

pub struct GroupTable (Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> Self {
        GroupTable(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, name: &String) -> Option<Arc<Group>> {
        self.0.lock()
        .expect("Should attain lock")
        .get(name)
        .cloned()
    }

    pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
        self.0.lock().expect("Should attain lock here.").entry(name.clone()).or_insert_with(|| Arc::new(Group::new(name)))
        .clone()
    }
}

    