
use super::Storage;

static mut STORAGE: *const Storage = 0 as *const Storage;

pub fn set_storage(storage:Box<Storage>){
    unsafe{STORAGE=Box::into_raw(storage);}
}

pub fn delete_storage(){
    unsafe{
        if STORAGE!=0 as *const Storage {
            let storage = Box::from_raw(STORAGE as *mut Storage);
            STORAGE = 0 as *const Storage;
        }
    }
}

pub fn get_storage() -> &'static Storage {
    unsafe{&*(STORAGE)}
}