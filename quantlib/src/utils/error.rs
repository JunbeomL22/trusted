use thiserror::Error;

#[derive(Error, Debug)]
pub enum VectorDataError {
    #[error(
        "An error occured at {struct_name}::{functino_name} \
        (other info: {other_info})"
    )]
    StructError {
        struct_name: &'static str,
        function_name: &'static str,
        other_info: String,
    },

    #[error("The length of value and dates must be the same in {function} of object {name}, value length: {value_length}, dates length: {dates_length}")]
    ValueAndDatesLengthMismatch {
        function: String,
        name: String,
        value_length: usize,
        dates_length: usize,
    },
    
    // Add other error variants as needed
}