use time::OffsetDateTime;
use crate::evaluation_date::EvaluationDate;
use crate::parameters::zero_curve::ZeroCurve;
use crate::assets::stock::Stock;
use crate::instruments::stock_futures::StockFutures;
use crate::definitions::Real;
use crate::pricing_engines::engine::Engine;
use crate::pricing_engines::calculation_result::CalculationResult;
use std::collections::HashMap;
use crate::instrument::Instrument;
//
use std::rc::Rc;
use std::cell::RefCell;

pub struct StockFuturesEngine {
    stock: Rc<RefCell<Stock>>,
    collateral_curve: Rc<RefCell<ZeroCurve>>, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    borrowing_curve: Rc<RefCell<ZeroCurve>>, // or repo
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    instruments: Vec<StockFutures>,
    results: HashMap<String, CalculationResult>, // Code -> CalculationResult
}

impl StockFuturesEngine {
    pub fn initialize(
        stock: Rc<RefCell<Stock>>,
        collateral_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        ) -> StockFuturesEngine {
        StockFuturesEngine {
            stock,
            collateral_curve,
            borrowing_curve,
            evaluation_date,
            instruments: vec![],
            results: HashMap::new(),
        }
    }

    pub fn set_instruments(&mut self, instruments: &Vec<StockFutures>) {
        // clone the instruments to self.instruments and initialize the results
        self.instruments = instruments.clone();
        // sanity check: the stock futures must have the same stock as its underlying
        for instrument in instruments {
            assert_eq!(&instrument.get_underlying_asset()[0], self.stock.borrow().get_name());
        }
        
        for instrument in instruments {
            let code = instrument.get_code().clone();
            let result = CalculationResult::default();
            self.results.insert(code, result);
        }

    }


    pub fn fair_forward(
        &self, 
        datetime: &OffsetDateTime
    ) -> Real {
        let stock_price = self.stock.borrow().get_last_price();
        let collateral_discount = self.collateral_curve.borrow().get_discount_factor_at_date(datetime);
        let borrowing_discount = self.borrowing_curve.borrow().get_discount_factor_at_date(datetime);
        let dividend_deduction_ratio = self.stock.borrow().get_dividend_deduction_ratio(datetime);

        let fwd: Real = stock_price * borrowing_discount / collateral_discount * dividend_deduction_ratio;
        fwd
    }
}

impl Engine for StockFuturesEngine {
    fn npv(&self) -> Real {
    }
}

