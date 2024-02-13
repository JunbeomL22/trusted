use time::OffsetDateTime;

pub trait Parameter {
    fn update(&mut self);
}

pub struct EvaluationDate {
    date: OffsetDateTime,
    observers: Vec<Box<dyn Parameter>>,
}
