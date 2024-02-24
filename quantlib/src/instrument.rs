use crate::parameters::currency::Currency;

pub trait Instrument {
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_currency(&self) -> &Currency;
}