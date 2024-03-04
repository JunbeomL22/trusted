use crate::data::observable::Observable;
use crate::evaluation_date::EvaluationDate;
// use std::rc::Rc;
// use std::cell::RefCell;

pub trait Parameter {
    fn update(&mut self, data: &dyn Observable) {}
    fn update_evaluation_date(&mut self, date: &EvaluationDate) {}  
    fn get_type_name(&self) -> String;
    fn get_name(&self) -> &String;
    fn get_address(&self) -> String;
}
