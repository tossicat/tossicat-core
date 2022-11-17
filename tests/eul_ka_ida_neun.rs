use library::{look_up_in_eul_ka_ida_neun, EUL};

#[test]
fn _look_up_in_eul_ka_ida_neun() {
    let temp = "네이버";
    let result = "를";
    assert_eq!(result, look_up_in_eul_ka_ida_neun(temp, EUL));
    // 마지막 글자가 영어가 나오는 경우
    let temp = "google";
    let result = "(을)를";
    assert_eq!(result, look_up_in_eul_ka_ida_neun(temp, EUL));
    // 괄호 안에 들어 있는 글자는 무시하고 바로 앞 글자가 마지막 글자가 됩니다.
    let temp = "넥슨(코리아)";
    let result = "을";
    assert_eq!(result, look_up_in_eul_ka_ida_neun(temp, EUL));
    // 숫자는 그 숫자를 한글로 발음하는 것으로 변환합니다.
    let temp = "비타500";
    let result = "을";
    assert_eq!(result, look_up_in_eul_ka_ida_neun(temp, EUL));
}
