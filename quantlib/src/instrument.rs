use crate::assets::currency::Currency;
use std::any::Any;
use crate::util::type_name;

pub trait Instrument: Any {
    fn as_any(&self) -> &dyn Any;
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_currency(&self) -> &Currency;
    fn type_name(&self) -> &str { type_name(&self) }
    fn clone_box(&self) -> Box<dyn Instrument>;
}

impl Clone for Box<dyn Instrument> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}


