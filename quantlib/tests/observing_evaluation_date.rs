#[cfg(test)]
mod tests {
    //use super::*;
    use anyhow::Result;
    use ndarray::array;
    use quantlib::currency::Currency;
    use quantlib::data::vector_data::VectorData;
    use quantlib::definitions::{DEFAULT_CLOSING_TIME, SEOUL_OFFSET};
    use quantlib::evaluation_date::EvaluationDate;
    use quantlib::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
    use quantlib::parameters::zero_curve::ZeroCurve;
    use std::cell::RefCell;
    use std::rc::Rc;
    use time;

    #[test]
    fn test_shared_evaluation_date() -> Result<()> {
        // Create a shared EvaluationDate
        let (h, m, s) = SEOUL_OFFSET;

        let evaluation_offset = time::UtcOffset::from_hms(h, m, s).unwrap();

        let evaluation_offsetdatetime = time::OffsetDateTime::new_in_offset(
            time::macros::date!(2021 - 01 - 01),
            DEFAULT_CLOSING_TIME,
            evaluation_offset,
        );

        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(
            evaluation_offsetdatetime.clone(),
        )));

        let spot = 1.0;
        // For constructing Zerocurve, make a vector data object which has two data points after the evaluation_date
        let value = array![0.03, 0.04];
        let dates = vec![
            evaluation_offsetdatetime + time::Duration::days(3),
            evaluation_offsetdatetime + time::Duration::days(365),
        ];
        let times = None;
        let market_datetime = evaluation_offsetdatetime.clone();
        let name = "zero curve data".to_string();
        let zero_curve_data = VectorData::new(
            value,
            Some(dates),
            times,
            Some(market_datetime),
            Currency::KRW,
            name.clone(),
            name.clone(),
        )
        .expect("Failed to create VectorData for zero curve");

        let _zero_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &zero_curve_data,
            String::from("KRWGOV"),
            "zero curve".to_string(),
        )
        .expect("Failed to create ZeroCurve");

        let zero_curve = Rc::new(RefCell::new(_zero_curve));

        // For constructing DiscreteRatioDividend, make a vector data object which has two data points after the evaluation_date
        let value = array![0.1, 0.2];
        let dates = vec![
            evaluation_offsetdatetime + time::Duration::days(3),
            evaluation_offsetdatetime + time::Duration::days(365),
        ];
        let times = None;
        let market_datetime = evaluation_offsetdatetime.clone();
        let name = "dividend amount data".to_string();
        let dividend_data = VectorData::new(
            value,
            Some(dates.clone()),
            times,
            Some(market_datetime),
            Currency::KRW,
            name.clone(),
            name.clone(),
        )
        .expect("Failed to create VectorData for dividend amount");

        let _dividend = DiscreteRatioDividend::new(
            evaluation_date.clone(),
            &dividend_data,
            spot,
            "dividend".to_string(),
            "dividend".to_string(),
        )
        .expect("Failed to create DiscreteRatioDividend");

        let dividend = Rc::new(RefCell::new(_dividend));
        evaluation_date
            .borrow_mut()
            .add_dividend_observer(dividend.clone());

        // test two dates
        let test_dates = vec![
            dates[0] + time::Duration::days(1),
            dates[1] + time::Duration::days(1),
        ];

        // obtain zero curve and dividend at the test_dates
        let mut first_zero_curve_values = vec![0.0, 0.0];
        let mut first_dividend_deductions = vec![0.0, 0.0];
        for i in 0..test_dates.len() {
            let date = test_dates[i];
            first_zero_curve_values[i] = zero_curve.borrow().get_discount_factor_at_date(&date)?;
            first_dividend_deductions[i] = dividend.borrow().get_deduction_ratio(&date)?;
        }

        // purturb the evaluation_date by three day
        *evaluation_date.borrow_mut() += "3D1sec";

        println!(
            "2) evaluation_date of dividend after purturbation: {:?}",
            dividend.borrow().get_evaluation_date_clone(),
        );

        let mut second_zero_curve_values = vec![0.0, 0.0];
        let mut second_dividend_deductions = vec![0.0, 0.0];

        for i in 0..test_dates.len() {
            let date = test_dates[i];
            second_zero_curve_values[i] = zero_curve.borrow().get_discount_factor_at_date(&date)?;
            second_dividend_deductions[i] = dividend.borrow().get_deduction_ratio(&date)?;
        }

        for i in 0..test_dates.len() {
            assert!(
                first_zero_curve_values[i] < second_zero_curve_values[i],
                "(for date {:?}) first_zero_curve_values: {:?}, second_zero_curve_values: {:?}",
                test_dates[i],
                first_zero_curve_values,
                second_zero_curve_values,
            );
            assert!(
                first_dividend_deductions[i] < second_dividend_deductions[i],
                "(for date {:?}) first_dividend_deductions: {:?}, second_dividend_deductions: {:?}",
                test_dates[i],
                first_dividend_deductions,
                second_dividend_deductions,
            );
        }

        // purturb back the evaluation_date by one day
        *evaluation_date.borrow_mut() -= "3D1sec";

        let mut third_zero_curve_values = vec![0.0, 0.0];
        let mut third_dividend_deductions = vec![0.0, 0.0];
        for i in 0..test_dates.len() {
            let date = test_dates[i];
            third_zero_curve_values[i] = zero_curve.borrow().get_discount_factor_at_date(&date)?;
            third_dividend_deductions[i] = dividend.borrow().get_deduction_ratio(&date)?;
        }

        // now the first and third should be the same
        for i in 0..test_dates.len() {
            assert!(
                (first_zero_curve_values[i] - third_zero_curve_values[i]) < 1.0e-10,
                "(for date {:?}) first_zero_curve_values: {:?}, third_zero_curve_values: {:?}",
                test_dates[i],
                first_zero_curve_values,
                third_zero_curve_values,
            );
            assert!(
                (first_dividend_deductions[i] - third_dividend_deductions[i]) < 1.0e-10,
                "(for date {:?}) first_dividend_deductions: {:?}, third_dividend_deductions: {:?}",
                test_dates[i],
                first_dividend_deductions,
                third_dividend_deductions,
            );
        }

        Ok(())
    }
}
