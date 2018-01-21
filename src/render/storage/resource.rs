
use std::marker::PhantomData;

use object_pool;
use self::object_pool::growable::Pool;

use storage;
use storage::ResourceSlot;

use resources::ResourceType;

use super::Storage;
use super::StorageError;

use failure::Error;

pub struct ResourcePool<R:storage::Resource, RR:Resource<R>> {
    pool:Pool<RR,RR>,
    _phantom_data:PhantomData<R>
}

pub trait Resource<R:storage::Resource>:Sized + 'static {
    fn get_type() -> ResourceType {
        R::get_type()
    }

    fn get_pool(storage:&mut Storage) -> &mut ResourcePool<R,Self>;

    fn new(resource:R, storage:&mut Storage) -> Result<Self,Error>;

    fn insert_to_storage(self, storage:&mut Storage) -> Result<(),Error> {
        Self::get_pool(storage).insert(self)
    }

    fn delete_from_storage(slot:ResourceSlot, storage:&mut Storage) -> Result<(),Error> {
        Self::get_pool(storage).delete(slot)
    }
}

impl<R:storage::Resource, RR:Resource<R>> ResourcePool<R,RR> {
    pub fn new() -> Self {
        ResourcePool {
            pool: Pool::new(),
            _phantom_data:PhantomData
        }
    }

    fn insert(&mut self, resource:RR) -> Result<(),Error> {
        self.pool.insert(resource);

        ok!()
    }

    fn delete(&mut self, slot:ResourceSlot) -> Result<(),Error> {
        self.pool.remove(slot);

        ok!()
    }

    pub fn get(&self, slot:ResourceSlot) -> Result<&RR,Error> {
        match self.pool.get(slot) {
            Some(resource) => ok!(resource),
            None => {
                let resource_id=format!("{:?} {}", RR::get_type(), slot);
                bail!(StorageError::NoResource(resource_id))
            }
        }
    }

    //TODO get
}