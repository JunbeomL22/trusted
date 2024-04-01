use crate::currency::{Currency, FxCode};

fn currencies_to_fx_codes(
    currencies: Vec<Currency>,
    must_include: Option<Vec<FxCode>>,
) -> Vec<FxCode> {
    let mut result = Vec::new();
    for cur1 in &currencies {
        for cur2 in &currencies {
            if cur1 != cur2 {
                result.push(FxCode::new(cur1.clone(), cur2.clone()));
            }
        }
    }

    match must_include {
        Some(must_include) => {
            for code in must_include {
                if !result.contains(&code) {
                    result.push(code);
                }
            }
        },
        None => {},
    }
    result
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::currency::Currency;
    use anyhow::Result;
    #[test]
    fn test_currencies_to_fx_codes() -> Result<()> {
        let currs = vec![
            Currency::USD,
            Currency::EUR,
            Currency::JPY,
            Currency::KRW,
        ];
        let must_include = vec![
            FxCode::new(Currency::KRW, Currency::KRW),
        ];
        let result = currencies_to_fx_codes(currs, None);
        for res in &result {
            println!("{:?}  ({:?})", res, res.to_string());
        }

        assert_eq!(result.len(), 12);
        assert!(result.contains(&FxCode::new(Currency::USD, Currency::EUR)));
        assert!(result.contains(&FxCode::new(Currency::EUR, Currency::USD)));
        assert!(result.contains(&FxCode::new(Currency::USD, Currency::JPY)));
        assert!(result.contains(&FxCode::new(Currency::JPY, Currency::USD)));
        assert!(result.contains(&FxCode::new(Currency::USD, Currency::KRW)));
        assert!(result.contains(&FxCode::new(Currency::KRW, Currency::USD)));
        assert!(result.contains(&FxCode::new(Currency::EUR, Currency::JPY)));
        assert!(result.contains(&FxCode::new(Currency::JPY, Currency::EUR)));
        assert!(result.contains(&FxCode::new(Currency::EUR, Currency::KRW)));
        assert!(result.contains(&FxCode::new(Currency::KRW, Currency::EUR)));
        assert!(result.contains(&FxCode::new(Currency::JPY, Currency::KRW)));
        assert!(result.contains(&FxCode::new(Currency::KRW, Currency::JPY)));

        Ok(())
    }
}