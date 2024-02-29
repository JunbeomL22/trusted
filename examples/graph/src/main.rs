use env_logger;
use quantlib::evaluation_date::EvaluationDate;
use quantlib::parameters::zero_curve::{self, ZeroCurve};
use quantlib::data::vector_data::VectorData;
use quantlib::time::calendar::{Calendar, NullCalendar};
use time::{OffsetDateTime, macros::datetime};
use quantlib::definitions::{Time, Real};
use std::rc::Rc;
use std::cell::RefCell;
use quantlib::utils::string_arithmetic::add_period;
use quantlib::enums::Compounding;
use quantlib::parameters::zero_curve_code::ZeroCurveCode;
use plotters::prelude::*;
use ndarray::array;
use ndarray::Array1;

fn plot_vectors(x_values: &Vec<Real>, y_values: &Vec<Real>, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Define the size of the chart
    let root_area = BitMapBackend::new(file_name, (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    // Define the range and labels of the chart
    let min_x = *x_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_x = *x_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_y = *y_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_y = *y_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Line Chart", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Combine x and y values into a single vector of points for plotting
    let data: Vec<(f32, f32)> = x_values.into_iter().zip(y_values.into_iter()).map(|(&x, &y)| (x, y)).collect();
    // Draw the line
    chart.draw_series(LineSeries::new(data, &RED))?;

    // Make sure the drawing is sent out to the bitmap
    root_area.present()?;

    Ok(())
}

fn main() {
    env_logger::init();
    let eval_dt = datetime!(2021-01-01 00:00:00 UTC);
    let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(eval_dt)));

    let param_dt = datetime!(2020-01-01 00:00:00 UTC);
    let dates = vec![
        add_period(&param_dt, "1M"),
        add_period(&param_dt, "1Y"),
        add_period(&param_dt, "2Y"),
        add_period(&param_dt, "3Y"),
        add_period(&param_dt, "5Y")
        ];

    let _data = VectorData::new(
        array![0.02, 0.025, 0.03, 0.035, 0.04],
        Some(dates.clone()), 
        None, 
        param_dt, 
        "vector data in test_zero_curve".to_string()
    );

    let data = Rc::new(RefCell::new(_data));

    let zero_curve = ZeroCurve::new(
        evaluation_date.clone(), 
        data.clone(), 
        ZeroCurveCode::Undefined, 
        "test".to_string()
    );

    // make a timestep from 0 to 10 years by 0.1
    let t_values: Vec<Time> = (0..=100).map(|i| i as Time / 10.0).collect::<Vec<Time>>();
    // let discount_values = zero_curve.get_vectorized_discount_factor_for_sorted_time(&t_values);
    // let short_rate_values = zero_curve.get_vectorized_short_rate_for_sorted_times(&t_values);
    let mut zero_curve_values = vec![0.0; t_values.len()];
    for i in 1..t_values.len() {
        zero_curve_values[i] = zero_curve.get_forward_rate_between_times(0.0, t_values[i], Compounding::Continuous);
    }
    plot_vectors(&t_values, &zero_curve_values, "./graphs/zero_rate_test.png").expect("Failed to plot vectors.");
}
