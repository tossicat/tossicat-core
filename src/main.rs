use tossicat::join_phonemes;
use tossicat::modify_sentence;
use tossicat::postfix;

fn main() {
    // 테스트
    println!("결과: {:?}", postfix("사과", "을"));
    println!("결과: {:?}", postfix("apple", "을"));
    println!("결과: {:?}", postfix("테스트", "처럼"));
    println!("결과: {:?}", postfix("천사1004", "은"));
    println!("결과: {:?}", postfix("테스트", "류현지"));
    println!("결과: {:?}", postfix("구글", "으로부터"));
    println!("결과: {:?}", postfix("토시캣", "보다"));
    println!("결과: {:?}", postfix("네이버", "코코"));
    println!("결과: {:?}", join_phonemes(['ㅋ', 'ㅔ', 'ㅅ']));
    let test = "{철수, 은} {영희,   과} {밥,  를} {나물, 코코} 먹습니다.";
    println!("테스트 문장: {test:?}");
    println!("테스트 결과 문장: {:?}", modify_sentence(test));
}
