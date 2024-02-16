use crate::parameter::Parameter;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

pub trait Observable: Any {
    fn notify_observers(&mut self);
    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>);
    fn as_any(&self) -> &dyn Any;
}