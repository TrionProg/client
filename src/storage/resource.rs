use std;

use object_pool;
use self::object_pool::growable::Pool;
use self::object_pool::growable::ID;

use std::marker::PhantomData;
use reactor::Mutex;

use failure::Error;

use render::RenderSender;

use super::StorageError;
use super::{Storage,get_storage};
use super::{CreateResourceCommand,DeleteResourceCommand};


pub type RefCounter=u32;

pub struct ResourcePool<R:Resource> {
    pool:Mutex< Pool<RefCounter,RefCounter> >,
    render_sender:RenderSender,
    _phantom_data:PhantomData<R>
}

pub struct ResourceID<R:Resource> {
    resource:*const R::RR, //Only on Render-Side
    id:ID
}

pub trait Resource:Sized {
    type RR;

    fn print_type() -> &'static str;
    fn get_pool(storage:&Storage) -> &ResourcePool<Self>;
    fn create_resource_command(self) -> CreateResourceCommand;
    fn delete_resource_command(resource_id:ResourceID<Self>) -> DeleteResourceCommand;

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

        let id=pool.insert(1);
        let resource_id=ResourceID::new(id);

        self.render_sender.send(R::create_resource_command(resource).into())?;

        ok!(resource_id)
    }

    fn delete(&self, resource_id:&ResourceID<R>) -> Result<(),Error> {
        mutex_lock!(&self.pool => pool);

        let delete={
            let ref_counter=pool.get_mut(resource_id.id).ok_or_else(||StorageError::NoResource(resource_id.print()))?;

            if *ref_counter==1 {
                true
            }else{
                *ref_counter-=1;
                false
            }
        };

        if delete {
            pool.remove(resource_id.id);

            self.render_sender.send(R::delete_resource_command(ResourceID::new(resource_id.id)).into())?;
        }

        ok!()
    }

    fn clone_resource_id(&self, resource_id:&ResourceID<R>) -> Result<(),Error> {
        mutex_lock!(&self.pool => pool);

        let ref_counter=pool.get_mut(resource_id.id).ok_or_else(||StorageError::NoResource(resource_id.print()))?;
        *ref_counter+=1;

        ok!()
    }

    fn get_ref_count(&self, resource_id:&ResourceID<R>) -> Result<RefCounter,Error> {
        mutex_lock!(&self.pool => pool);

        let ref_counter=pool.get_mut(resource_id.id).ok_or_else(||StorageError::NoResource(resource_id.print()))?;

        ok!(*ref_counter)
    }
}

impl<R:Resource> ResourceID<R> {
    pub fn new(id:ID) -> Self {
        ResourceID{
            resource:0 as *const R,
            id
        }
    }

    pub fn print(&self) -> String {
        format!("{} #{}", R::print_type(), self.id.slot_index)
    }

    pub fn get_ref_count(&self) -> RefCounter {
        R::get_pool(get_storage()).get_ref_count(self).unwrap()
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
            id:self.id
        }
    }
}