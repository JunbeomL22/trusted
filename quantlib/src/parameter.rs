use crate::data::observable::Observable;
use crate::evaluation_date::EvaluationDate;
use anyhow::Result;
// use std::rc::Rc;
// use std::cell::RefCell;

pub trait Parameter {
    fn update(&mut self, _data: &dyn Observable) -> Result<()> { Ok(()) }
    fn update_evaluation_date(&mut self, _date: &EvaluationDate) -> Result<()> { Ok(()) }  
    fn get_type_name(&self) -> &'static str;
    fn get_name(&self) -> &String;
    fn get_address(&self) -> String {
        let address = format!("{:p}", &self);
        address
    }
}
