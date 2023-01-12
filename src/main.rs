use tossicat::join_phonemes;
use tossicat::modify_sentence;
use tossicat::pick;
use tossicat::postfix;
use tossicat::value_verifier;

fn main() {
    // 테스트
    println!("결과: {}", postfix("사과", "을"));
    println!("결과: {}", pick("사과", "을"));
    println!("결과: {}", postfix("테스트", "처럼"));
    println!("결과: {}", postfix("천사1004", "은"));
    println!("결과: {}", pick("테스트", "류현지"));
    println!("결과: {}", pick("구글", "으로부터"));
    println!("결과: {}", pick("토시캣", "보다"));
    println!("결과: {}", pick("네이버", "에서부터"));
    println!("결과: {:?}", value_verifier("apple", "은"));
    println!("결과: {:?}", value_verifier("apple", "는"));
    println!("결과: {:?}", value_verifier("apple", "is"));
    // 아래 것은 '누가'가 나와야 합니다.
    println!("결과: {}", postfix("누구", "이"));
    println!("결과: {}", join_phonemes(['ㅋ', 'ㅔ', 'ㅅ']));
    let test = "{철수, 은} {영희,   과} {밥,  를} 먹습니다.";
    println!("테스트 문장: {}", test);
    let testing = modify_sentence(test);
    println!("결과 문장: {:?}", testing);
    let test = "{철수, 은} {영희,   과} {밥,  를} {나물,} 먹습니다.";
    println!("테스트 문장: {}", test);
    let testing = modify_sentence(test);
    println!("결과 문장: {:?}", testing);
}
