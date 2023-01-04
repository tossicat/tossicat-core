use tossicat::brackets;
use tossicat::join_phonemes;
use tossicat::pick;
use tossicat::postfix;
use tossicat::verifiers;

fn main() {
    // 테스트
    println!("결과: {}", postfix("사과", "을"));
    println!("결과: {}", pick("사과", "을"));
    println!("결과: {}", postfix("테스트", "처럼"));
    println!("결과: {}", postfix("천사1004", "은"));
    println!("결과: {}", pick("테스트", "은"));
    println!("결과: {}", pick("구글", "으로부터"));
    println!("결과: {}", pick("토시캣", "보다"));
    println!("결과: {}", pick("네이버", "에서부터"));
    println!("결과: {:?}", verifiers("apple", "은"));
    println!("결과: {:?}", verifiers("apple", "는"));
    println!("결과: {:?}", verifiers("apple", "is"));
    // 아래 것은 '누가'가 나와야 합니다.
    println!("결과: {}", postfix("누구", "이"));
    println!("결과: {}", join_phonemes(['ㅋ', 'ㅔ', 'ㅅ']));
    let test = "{철수, 은} {영희,   과} {밥,  를} {,,} 먹습니다.";
    println!("테스트 문장: {}", test);
    let temp = brackets(test);
    println!("temp 결과: {:?}", temp);
    for item in temp.1 {
        println!("결과: {}", postfix(&item.0, &item.1));
    }
    println!("결과: {:?}", test.replace("{철수, 은}", "철수는"));
}
