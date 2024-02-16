use crate::data::observable::Observable;

pub trait Parameter {
    fn update(&mut self, data: &dyn Observable);
}
