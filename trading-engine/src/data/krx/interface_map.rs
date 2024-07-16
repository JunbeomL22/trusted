use crate::info;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

pub static KRX_TR_CODE_MAP: Lazy<FxHashMap<&'static [u8], &'static str>> = Lazy::new(|| {
    let mut m = FxHashMap::default();

    // 일반채권, 국고채권 우선호가	
    // Regular Bonds, KTB_Quote
    ["B601B", // BND
    "B601K", // KTS
    ].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0023");
    });


    // 채권 체결	
    // Bonds_Order Filled
    ["A301B", // BND 
    "A301M", // SMB
    "A301K", // KTS
    ].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0027");
    });

    // 일반채권, 국고채권 체결 + 우선호가	
    // General Bonds, KTB Order filled + Quote
    ["G701B", // BND
    "G701K", // KTS
    ].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0029");
    });

    // 증권 우선호가 (MM/LP호가 제외)
    // Securities Quote (Excluding MM/LP Quotes)
    ["B601S", // STK
     "B601Q", // KSQ
     "B601X", // KNX
    ].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0002");
    });

    // 증권 우선호가 (MM/LP호가 포함)
    // Securities Quote (Including MM/LP Quotes)
    ["B702S", "B703S", "B704S", // STK
    ].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0003");
    });

    // 증권 체결
    // Securities Trade
    ["A301S", // STK (A)
    "A302S", "A303S", "A304S" // STK (C)
    "A301Q", // KSQ
    "A301X", // KNX
    ].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0004");
    });

    // 파생 체결
    // Derivative Trade
    ["A301F", "A302F", "A303F", "A304F", "A305F", "A306F", "A307F", "A308F", 
    "A309F", "A310F", "A311F", "A312F", "A313F", "A315F", "A316F", "A314F"].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0036");
    });
    // 파생 우선호가 (우선호가 5단계)
    // Derivative Best Bid/Ask (5 levels)
    ["B601F", "B602F", "B603F", "B606F", "B607F", "B608F", "B609F", 
    "B610F", "B611F", "B612F", "B613F", "B615F", "B616F", "B614F"].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0034");
    });

    // 파생 우선호가 (우선호가 10단계)
    // Derivative Best Bid/Ask (10 levels)
    ["B604F", "B605F"].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0035");
    });

    // 파생 체결 + 우선호가 (우선호가 5단계)
    // Derivative Trade + Best Bid/Ask (5 levels)
    ["G701F", "G702F", "G703F", "G706F", "G707F", "G708F", 
    "G709F", "G710F", "G711F", "G712F", "G713F", "G715F", "G716F"].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0037");
    });

    // 파생 체결 + 우선호가 (우선호가 10단계)
    // Derivative Trade + Best Bid/Ask (10 levels)
    ["G704F",  // Stock Futures
    "G705F", // Stock Options
    ].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0038");
    });

    info!("initializing..KRX_TR_CODE_MAP is created");
    m
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        println!("TR-Code Interface");
        for (tr_code, interface_id) in KRX_TR_CODE_MAP.iter() {
            println!(
                "{:?} -> {:?}",
                std::str::from_utf8(tr_code).unwrap(),
                interface_id
            );
        }

        let tr_code = "A301F".as_bytes();
        let interface_id = KRX_TR_CODE_MAP.get(tr_code).unwrap();
        assert_eq!(interface_id, &"IFMSRPD0036");
    }
}
