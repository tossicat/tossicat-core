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
}
