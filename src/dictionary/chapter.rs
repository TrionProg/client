
use reactor::Mutex;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Display;

use failure::Error;
use storage::{Resource,ResourceID};

use super::DictionaryError;

pub trait ResourceIDTrait:Clone + Drop + Display {
    fn get_ref_count(&self) -> u32;
}

impl<R:Resource> ResourceIDTrait for ResourceID<R> {
    fn get_ref_count(&self) -> u32 {
        self.get_ref_count()
    }
}

pub struct Chapter<ID:ResourceIDTrait> {
    hash_map:Mutex< HashMap<String,ID> >,
}

impl<ID:ResourceIDTrait> Chapter<ID> {
    pub fn new() -> Self {
        let chapter=Chapter {
            hash_map:Mutex::new(HashMap::with_capacity(64))
        };

        chapter
    }

    pub fn insert(&self, name:String, resource_id:ID) -> Result<(),Error> {
        mutex_lock!(&self.hash_map => hash_map);

        match hash_map.entry(name) {
            Entry::Vacant(e) => {e.insert(resource_id); ok!()},
            Entry::Occupied(e) => bail!(DictionaryError::AlreadyExists(e.key().to_string())),
        }
    }

    pub fn get(&self, name:&str) -> Result<ID,Error> {
        mutex_lock!(&self.hash_map => hash_map);

        match hash_map.get(name) {
            Some(resource_id) => ok!((*resource_id).clone()),
            None => bail!(DictionaryError::NoResource(name.to_string())),
        }
    }

    pub fn delete(&self, name:&str) -> Result<(),Error> {
        mutex_lock!(&self.hash_map => hash_map);

        match hash_map.get(name) {
            Some(resource_id) => {
                if resource_id.get_ref_count() > 1 {
                    println!("Warn {} is already in use(reference counter>1)",resource_id);//TODO check counter and warn!
                }
            }
            None => bail!(DictionaryError::NoResource(name.to_string())),
        }

        hash_map.remove(name).unwrap();

        ok!()
    }
}

//Chapter for resources..