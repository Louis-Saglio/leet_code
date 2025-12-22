fn convert_decimal(val: i32, unit: &str, five: &str, ten: &str) -> String {
    match val {
        0 => "".to_string(),
        1 => format!("{}", unit),
        2 => format!("{}{}", unit, unit),
        3 => format!("{}{}{}", unit, unit, unit),
        4 => format!("{}{}", unit, five),
        5 => format!("{}", five),
        6 => format!("{}{}", five, unit),
        7 => format!("{}{}{}", five, unit, unit),
        8 => format!("{}{}{}{}", five, unit, unit, unit),
        9 => format!("{}{}", unit, ten),
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
pub fn int_to_roman(num: i32) -> String {
    let mut result = String::new();
    let mut num = num;
    let val = num / 1000;
    let thousands = match val {
        0 => "",
        1 => "M",
        2 => "MM",
        3 => "MMM",
        _ => unreachable!(),
    };
    result.push_str(thousands);
    num -= val * 1000;
    let data = [
        (100, "C", "D", "M"),
        (10, "X", "L", "C"),
        (1, "I", "V", "X"),
    ];
    for (magnitude, unit, five, ten) in data {
        let decimal = num / magnitude;
        let roman = convert_decimal(decimal, unit, five, ten);
        result.push_str(&roman);
        num -= decimal * magnitude;
    }
    result
}

#[test]
fn test() {
    assert_eq!(int_to_roman(3), "III".to_string());
    assert_eq!(int_to_roman(4), "IV".to_string());
    assert_eq!(int_to_roman(9), "IX".to_string());
    assert_eq!(int_to_roman(58), "LVIII".to_string());
    assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
}
