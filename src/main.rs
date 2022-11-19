use tossicat::pick;
use tossicat::postfix;
use tossicat::verifiers;

fn main() {
    // 테스트
    println!("결과: {}", postfix("테스트", "은"));
    println!("결과: {}.", pick("테스트", "은"));
    println!("결과: {:?}", verifiers("apple", "은"));
    println!("결과: {:?}", verifiers("apple", "는"));
    println!("결과: {:?}", verifiers("apple", "is"));
}
