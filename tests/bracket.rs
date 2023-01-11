use tossicat::bracket::modify_pairs;

#[test]

fn _modify_pairs() {
    let temp: String = "{철수, 은} {영희,   과} {밥,  를} 먹습니다.".to_owned();
    let result = Ok(vec![
        ("철수, 은".to_owned(), "철수".to_owned(), "은".to_owned()),
        ("영희,   과".to_owned(), "영희".to_owned(), "과".to_owned()),
        ("밥,  를".to_owned(), "밥".to_owned(), "를".to_owned()),
    ]);
    assert_eq!(result, modify_pairs(&temp));
}
