
use failure::Error;
use location::Location;

pub trait Instance {
    fn set_location(&self, location:&Location) -> Result<(),Error>;
}