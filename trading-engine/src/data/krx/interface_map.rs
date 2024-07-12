use crate::info;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

pub static KRX_TR_CODE_MAP: Lazy<FxHashMap<&'static [u8], &'static str>> = Lazy::new(|| {
    let mut m = FxHashMap::default();

    // IFMSRPD0036
    // 파생 체결
    [
        "A301F", "A302F", "A303F", "A304F", "A305F", "A306F", "A307F", "A308F", "A309F", "A310F",
        "A311F", "A312F", "A313F", "A315F", "A316F", "A314F", "G701F", "G702F", "G703F", "G706F",
        "G707F", "G708F", "G709F", "G710F", "G711F", "G712F", "G713F", "G715F", "G716F", "G714F",
    ]
    .iter()
    .for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0036");
    });

    // IFMSRPD0037
    // 파생 체결 + 우선호가 (우선호가 5단계)
    ["G704F", "G705F"].iter().for_each(|&code| {
        m.insert(code.as_bytes(), "IFMSRPD0037");
    });

    // IFMSRPD0038
    // 파생 체결 + 우선호가 (우선호가 10단계)
    ["G704F", "G705F"].iter().for_each(|&code| {
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
