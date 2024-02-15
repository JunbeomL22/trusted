use crate::parameter::Parameter;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Observable {
    fn notify_observers(&mut self);
    fn add_observer(&mut self, observer: Rc<RefCell<dyn Parameter>>);
}