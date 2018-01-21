
use resources::*;

use super::Chapter;

pub struct Dictionary {
    pub rgba_textures:Chapter<RgbaTextureID>,
}

impl Dictionary {
    pub fn initialize() {
        use super::global::set_dictionary;

        let dictionary=Box::new(Dictionary::new());
        set_dictionary(dictionary);
    }

    fn new() -> Self{
        Dictionary{
            rgba_textures:Chapter::new(),
        }
    }

    pub fn delete(){
        use super::global::delete_dictionary;
        delete_dictionary();
    }
}