use crate::assets::currency::Currency;
use std::any::Any;

pub trait Instrument: Any {
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_currency(&self) -> &Currency;
}
