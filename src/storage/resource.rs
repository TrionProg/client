use std;

use object_pool;
use self::object_pool::growable::Pool;
pub use self::object_pool::growable::ID as ResourceSlot;

use std::marker::PhantomData;
use reactor::Mutex;

use failure::Error;

use render;
use render::RenderSender;
use render::{RenderCommand,StorageCommand};

use resources::ResourceType;

use super::StorageError;
use super::{Storage,get_storage};


pub type RefCounter=u32;

pub struct ResourcePool<R:Resource> {
    pool:Mutex< Pool<RefCounter,RefCounter> >,
    render_sender:RenderSender,
    _phantom_data:PhantomData<R>
}

pub struct ResourceID<R:Resource> {
    resource:*const R::RR, //Only on Render-Side
    slot:ResourceSlot
}

pub trait Resource:Sized + Send + 'static {
    type RR:render::storage::Resource<Self>;

    fn get_type() -> ResourceType;
    fn get_pool(storage:&Storage) -> &ResourcePool<Self>;

    fn insert_resource_command(self) -> StorageCommand<Self> {
        StorageCommand::Insert(self)
    }

    fn delete_resource_command(slot:ResourceSlot) -> StorageCommand<Self> {
        StorageCommand::Delete(slot)
    }

    fn insert_to_storage(self) -> Result<ResourceID<Self>,Error> {
        Self::get_pool(get_storage()).insert(self)
    }
}

impl<R:Resource> ResourcePool<R> {
    pub fn new(render_sender:RenderSender) -> Self {
        ResourcePool {
            pool:Mutex::new(Pool::new()),
            render_sender,
            _phantom_data:PhantomData
        }
    }

    fn insert(&self, resource:R) -> Result<ResourceID<R>,Error> {
        mutex_lock!(&self.pool => pool);

        let slot=pool.insert(1);
        let resource_id=ResourceID::new(slot);

        self.render_sender.send(R::insert_resource_command(resource).into())?;

        ok!(resource_id)
    }

    fn delete(&self, resource_id:&ResourceID<R>) -> Result<(),Error> {
        mutex_lock!(&self.pool => pool);

        let delete={
            let ref_counter=pool.get_mut(resource_id.slot).ok_or_else(||StorageError::NoResource(resource_id.print()))?;

            if *ref_counter==1 {
                true
            }else{
                *ref_counter-=1;
                false
            }
        };

        if delete {
            pool.remove(resource_id.slot);

            self.render_sender.send(R::delete_resource_command(resource_id.slot).into())?;
        }

        ok!()
    }

    fn clone_resource_id(&self, resource_id:&ResourceID<R>) -> Result<(),Error> {
        mutex_lock!(&self.pool => pool);

        let ref_counter=pool.get_mut(resource_id.slot).ok_or_else(||StorageError::NoResource(resource_id.print()))?;
        *ref_counter+=1;

        ok!()
    }

    fn get_ref_count(&self, resource_id:&ResourceID<R>) -> Result<RefCounter,Error> {
        mutex_lock!(&self.pool => pool);

        let ref_counter=pool.get_mut(resource_id.slot).ok_or_else(||StorageError::NoResource(resource_id.print()))?;

        ok!(*ref_counter)
    }
}

impl<R:Resource> ResourceID<R> {
    pub fn new(slot:ResourceSlot) -> Self {
        ResourceID{
            resource:0 as *const R::RR,
            slot
        }
    }

    pub fn print(&self) -> String {
        format!("{}", self)
    }

    pub fn get_ref_count(&self) -> RefCounter {
        R::get_pool(get_storage()).get_ref_count(self).unwrap()
    }

    pub fn index(&mut self, storage:&mut render::Storage) -> Result<(),Error> {
        use render::storage::resource::Resource;

        if self.resource == 0 as *const R::RR {
            let res=R::RR::get_pool(storage).get(self.slot)?;
            self.resource=res as *const R::RR;
        }

        ok!()
    }
}

unsafe impl<R:Resource> Send for ResourceID<R> {}

impl<R:Resource> std::fmt::Display for ResourceID<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} #{}", R::get_type(), self.slot.slot_index)
    }
}


impl<R:Resource> Drop for ResourceID<R> {
    fn drop(&mut self) {
        R::get_pool(get_storage()).delete(self).unwrap();
    }
}

impl<R:Resource> Clone for ResourceID<R> {
    fn clone(&self) -> Self {
        R::get_pool(get_storage()).clone_resource_id(self).unwrap();

        ResourceID{
            resource:self.resource,
            slot:self.slot
        }
    }
}