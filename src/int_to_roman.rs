/**
12. Integer to Roman

Seven different symbols represent Roman numerals with the following values:
Symbol	Value
I	1
V	5
X	10
L	50
C	100
D	500
M	1000

Roman numerals are formed by appending the conversions of decimal place values from highest to lowest. Converting a decimal place value into a Roman numeral has the following rules:

If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input, append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol, for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX. Only the following subtractive forms are used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).
Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5 (V), 50 (L), or 500 (D) multiple times. If you need to append a symbol 4 times use the subtractive form.

Given an integer, convert it to a Roman numeral.


Example 1:

Input: num = 3749

Output: "MMMDCCXLIX"

Explanation:

3000 = MMM as 1000 (M) + 1000 (M) + 1000 (M)
 700 = DCC as 500 (D) + 100 (C) + 100 (C)
  40 = XL as 10 (X) less of 50 (L)
   9 = IX as 1 (I) less of 10 (X)
Note: 49 is not 1 (I) less of 50 (L) because the conversion is based on decimal places

Example 2:

Input: num = 58

Output: "LVIII"

Explanation:

50 = L
 8 = VIII

Example 3:

Input: num = 1994

Output: "MCMXCIV"

Explanation:

1000 = M
 900 = CM
  90 = XC
   4 = IV


Constraints:

    1 <= num <= 3999

**/

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
    let (result, _) = [
        (100, "C", "D", "M"),
        (10, "X", "L", "C"),
        (1, "I", "V", "X"),
    ]
    .into_iter()
    .fold(
        (result, num),
        |(result, num), (magnitude, unit, five, ten)| {
            let decimal = num / magnitude;
            let result = format!("{}{}", result, convert_decimal(decimal, unit, five, ten));
            let num = num - decimal * magnitude;
            (result, num)
        },
    );
    result
}

#[test]
fn test() {
    assert_eq!(int_to_roman(3), "III".to_string());
    assert_eq!(int_to_roman(4), "IV".to_string());
    assert_eq!(int_to_roman(9), "IX".to_string());
    assert_eq!(int_to_roman(58), "LVIII".to_string());
    assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    assert_eq!(int_to_roman(1005), "MV".to_string());
}
