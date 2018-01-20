
use super::Dictionary;

static mut DICTIONARY: *const Dictionary = 0 as *const Dictionary;

pub fn set_dictionary(dictionary:Box<Dictionary>){
    unsafe{DICTIONARY=Box::into_raw(dictionary);}
}

pub fn delete_dictionary(){
    unsafe{
        if DICTIONARY!=0 as *const Dictionary {
            let dictionary = Box::from_raw(DICTIONARY as *mut Dictionary);
            DICTIONARY = 0 as *const Dictionary;
        }
    }
}

pub fn get_dictionary() -> &'static Dictionary {
    unsafe{&*(DICTIONARY)}
}