use library::find_last_letter;

#[test]

fn _find_last_letter() {
    let temp = "넥슨(코리아)";
    let result = '슨';
    assert_eq!(result, find_last_letter(temp));

    let temp = "비타500";
    let result = '백';
    assert_eq!(result, find_last_letter(temp));

    // let temp = "naver";
    // let result = 'r';
    // assert_eq!(result, find_last_letter(temp));

    // let temp = "diablo";
    // let result = 'o';
    // assert_eq!(result, find_last_letter(temp));
}
