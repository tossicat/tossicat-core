use tossicat::change_num_to_hangeul;

#[test]
fn _change_num_to_hangeul() {
    let test = "0";
    let result = "영";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "1";
    let result = "일";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "2";
    let result = "이";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "10";
    let result = "십";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "11";
    let result = "십일";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "10000";
    let result = "만";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "10001";
    let result = "만일";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "10010";
    let result = "만십";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "20100";
    let result = "이만백";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "100009";
    let result = "십만구";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "12345";
    let result = "만이천삼백사십오";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "100000000";
    let result = "일억";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "1000000000";
    let result = "십억";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "10000000000000000000000000000000000000000000000000";
    let result = "십극";
    assert_eq!(result, change_num_to_hangeul(test));

    let test = "90000000000000000000000000000000000000000000000000";
    let result = "구십극";
    assert_eq!(result, change_num_to_hangeul(test));
}
