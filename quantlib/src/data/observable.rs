use crate::parameter::Parameter;

pub trait Observable {
    fn notify_observers(&mut self);
    fn add_observer(&mut self, observer: Box<dyn Parameter>);
}