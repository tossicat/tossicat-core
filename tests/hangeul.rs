use tossicat::{join_phonemes, modify_finall_jamo, split_phonemes};

#[test]
fn _modify_finall_jamo() {
    let temp = '정';
    assert_eq!('점', modify_finall_jamo(temp, 'ㅁ'));
    let temp = '감';
    assert_eq!('강', modify_finall_jamo(temp, 'ㅇ'));
    //
    // 이하 테스트들은 모두 문제가 있어 입력된 것을 그대로 반환한다.
    // 이건 초성으로는 쓰이는 자음이지만, 총성으로는 쓰이지 않는다.
    assert_eq!('감', modify_finall_jamo('감', 'ㄸ'));
    // 당연히 한글이 아니다.
    assert_eq!('a', modify_finall_jamo('a', 'ㅁ'));
    // 이것도 당연히
    assert_eq!('😀', modify_finall_jamo('😀', 'ㅁ'));
}

#[test]
fn _join_phonemes() {
    let temp: [char; 3] = ['ㅅ', 'ㅓ', 'ㅂ'];
    let result = '섭';
    assert_eq!(result, join_phonemes(temp));

    let temp: [char; 3] = ['ㄸ', 'ㅗ', 'ㅁ'];
    let result = '똠';
    assert_eq!(result, join_phonemes(temp));

    let temp: [char; 3] = ['ㄸ', 'ㅗ', 'ㅁ'];
    let result = '똠';
    assert_eq!(result, join_phonemes(temp));

    let temp: [char; 3] = ['ㄸ', 'ㅗ', 'ㅁ'];
    let result = '똠';
    assert_eq!(result, join_phonemes(temp));

    let temp: [char; 3] = ['ㄸ', 'ㅗ', 'ㅁ'];
    let result = '똠';
    assert_eq!(result, join_phonemes(temp));

    let temp: [char; 3] = ['ㄸ', 'ㅗ', 'ㅁ'];
    let result = '똠';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['ㄱ', 'ㅏ', ' '];
    let result = '가';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['a', ' ', ' '];
    let result = 'a';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['a', 'b', ' '];
    let result = 'a';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['a', 'b', 'c'];
    let result = 'a';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['😀', ' ', ' '];
    let result = '😀';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['😀', 'a', ' '];
    let result = '😀';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['😀', 'ㅏ', ' '];
    let result = '😀';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['ㄱ', 'a', ' '];
    let result = 'ㄱ';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['ㄱ', 'ㄴ', 'ㄷ'];
    let result = 'ㄱ';
    assert_eq!(result, join_phonemes(temp));

    let temp = ['ㅊ', 'ㄴ', 'ㅓ'];
    let result = 'ㅊ';
    assert_eq!(result, join_phonemes(temp));
}

// def test_join_phonemes():
//     assert join_phonemes(u'ㅅ', u'ㅓ', u'ㅂ') == u'섭'
//     assert join_phonemes((u'ㅅ', u'ㅓ', u'ㅂ')) == u'섭'
//     assert join_phonemes(u'ㅊ', u'ㅠ') == u'츄'
//     assert join_phonemes(u'ㅊ', u'ㅠ', u'') == u'츄'
//     assert join_phonemes((u'ㅊ', u'ㅠ')) == u'츄'
//     with pytest.raises(TypeError):
//         join_phonemes(u'ㄷ', u'ㅏ', u'ㄹ', u'ㄱ')

#[test]
fn _split_phonemes() {
    let temp = '쏚';
    let result = ['ㅆ', 'ㅗ', 'ㄲ'];
    assert_eq!(result, split_phonemes(temp));

    let temp = '섭';
    let result = ['ㅅ', 'ㅓ', 'ㅂ'];
    assert_eq!(result, split_phonemes(temp));

    let temp = '투';
    let result = ['ㅌ', 'ㅜ', ' '];
    assert_eq!(result, split_phonemes(temp));

    let temp = '똠';
    let result = ['ㄸ', 'ㅗ', 'ㅁ'];
    assert_eq!(result, split_phonemes(temp));

    let temp = '가';
    let result = ['ㄱ', 'ㅏ', ' '];
    assert_eq!(result, split_phonemes(temp));
}
