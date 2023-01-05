# TossiCat core

![This is an image](../doc/logo/tossicat_core_logo.png)

이 프로젝트는 [tossi](https://github.com/what-studio/tossi)에서 영감을 받았습니다. 파이썬으로 구현된 앞의 **tossi**처럼 이 프로젝트도 임의의 단어와 그 단어에 붙일 조사(즉 토시)를 입력하면, 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔서 반환해 줍니다.

## 사용법

사용하는 방법은 다음과 같습니다. 현재 프로젝트 안에 있는 `Cargo.toml` 파일에서 `[dependencies]` 다음에 `tossicat = "0.3.2"`을 다음과 같이 추가해 주세요.

```toml
[dependencies]

tossicat = "0.3.2"
```

이제 여러분의 코드에서 다음과 같이 `tossicat`에 들어 있는 함수를 다음과 같이 사용할 수 있습니다.

```rust
use tossicat::pick;
use tossicat::postfix;

fn main() {
    println!("결과: {}", postfix("사과", "을"));
    println!("결과: {}", pick("사과", "을"));
}
```

실행 결과는 다음과 같습니다.

```cmd
결과: 사과를
결과: 를
```

자세한 내용은 [https://crates.io/crates/tossicat](https://crates.io/crates/tossicat) 참고하세요.

## 왜 이 라이브러리가 필요할까?

왜 단어마다 토시를 바꿔서 붙여야 할까요? 예를 들어 봅시다.  
'사과'라는 단어에 목적격 조사(을/를)를 붙일 경우, '을'이 아닌 '를'을 붙여야 합니다. 왜냐하면 목적격조사는 바로 앞 글자에 받침이 없으면 '을', 받침이 있다면 '를'을 사용해야 하기 때문입니다. 물론 '사과'라는 단어에 '처럼'이라는 조사를 붙일 경우에는 이런 변화를 고려할 필요가 없습니다. 왜냐하면 '처럼'이라는 조사는 어떠한 단어에도 변칙 없이 붙일 수 있기 때문입니다. 이처럼 조사에 따라 바꿔 붙여야 하는 경우가 있고, 그렇게 할 필요가 없는 경우가 있을 수 있습니다. 바꿔 붙여야 하는 경우에도 어떤 규칙을 사용하고 있는지 선택해야 합니다.

물론 한국어를 사용하는 한국인 입장에서는 학교 교육을 받으면서 이런 것들을 쉽게 구별해 꽤 정확하게 처리할 수 있습니다. 그러나 외국인 입장에서 보면 그리 쉽지 않습니다. 더 중요한 문제는 앱 안에서 문장을 조합해서 사용자에게 표현하고자 할 때, 이런 특정 단어에 토시를 붙이는 것을 자동화하기 위해서는 이것을 자동화하는 라이브러리가 필요하게 됩니다. 바로 이 프로젝트가 이런 경우에 필요한 것입니다.

앞의 코드에서 `postfix("사과", "을")`의 실행 결과를 보면, `결과: 사과를`을 인 것을 보면 `postfix()` 함수가 적절한 값을 찾아 "사과"라는 단어에 붙여 반환한 것을 확인할 수 있습니다. 두 번째로 실행한 `pick()` 함수는 적절한 토시만 반환합니다.

이 프로젝트는 `Rust`로 작성하고 있습니다. 자세한 내용은 아래 내용을 참고하세요. 참고로 이 프로젝트의 원조는 [LOPES-HUFS-tossi_for_rust](https://github.com/LOPES-HUFS/tossi_for_rust)이었습니다. 그러나 확장성을 고려하지 않고 프로젝트를 진행하다 보니 한계에 도달하게 되었습니다. 그래서 각 기능을 분리하고 확장하고자 [TossiCat](https://github.com/tossicat)이라는 조직(organization)을 만들고 목적에 맞게 코드를 각각의 저장소들(repositories)로 나눠 코딩하고자 원 코드를 옮겨서 다시 시작하게 되었습니다.

## 구현 함수

여기에서는 다음과 같은 2가지 기능을 지원합니다.

- `pick(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 반환합니다.
- `postfix(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 덧붙여 반환합니다.

## 이 프로젝트에서 처리 가능한 토시(조사)에 대하여

이 프로젝트는 토시를 두 종류로 분류하고 있습니다.

1. 변환을 고려해야 하는 토시들
2. 변환할 필요가 없는 토시들

여기서 "변환할 필요가 없는 토시들"은 어떤 단어에 붙여서 사용해도 문제가 되지 않습니다. 그러나 "변환 가능한 토시들"은 이 토시들이 어떤 단어에 붙일 것인가에 따라 같은 의미를 가진 다른 토시로 변환해야 합니다. 이 프로젝트는 이 두 종류의 토시를 다 처리할 수 있기 때문에, 어떤 토시를 변환해야 하는지, 어떤 토시는 변환할 필요가 없는 것인지를 판단하고 변환해야 할 토시라면 적절하게 변환해 줍니다.

### 처리할 수 있는 토시 목록

현재 이 프로젝트에서 토시(tossi)는 다음과 같습니다. 지속적으로 추가할 예정입니다.

1. 변환을 고려해야 하는 토시들, 46개
2. 변환할 필요가 없는 토시들, 31개

이 둘을 합쳐서 총 77개의 토시를 처리할 수 있습니다. 여기에 외국어 단어가 입력되었을 경우 그 단어의 발음을 정확하게 알 수 없기 때문에, "(을)를"과 같이 토시를 병기해 변환하는 경우가 일어날 수 있기 때문에 내부적으로 처리할 수 있는 토시 숫자는 훨씬 많습니다. 이와 관련된 자세한 내용은 [RELEASES.md](https://github.com/tossicat/tossicat-core/blob/main/RELEASES.md)를 참고하세요.

### 변환을 고려해야 하는 토시들

"가", "고", "과", "나", "나마", "는", "니", "다", "든", "든가", "든지", "라도", "라야", "란", "랑", "로", "로부터", "로서", "로써", "를", "며", "야말로", "여", "와", "으로", "으로부터", "으로서", "으로써", "은", "을", "이", "이고", "이나", "이나마", "이니", "이다", "이든", "이든가", "이든지", "이라도", "이라야", "이란", "이랑", "이며", "이야말로", "이여"

### 변환할 필요가 없는 토시들

"같이", "거나", "게", "까지", "께", "께서", "대로", "도", "마냥", "마다", "마저", "만", "밖에", "보다", "부터", "뿐", "에", "에게", "에게로", "에게서", "에다가", "에서", "에서부터", "의", "이다", "조차", "처럼", "커녕", "하고", "한테", "한테서"

## 이 프로젝트를 빌드하기

이 프로젝트를 빌드하기 위해서는 다음 명령어를 실행하면 됩니다.

```console
cargo build --release
```

빌드하고 나면 `tossicat-core/target/release`에 `tossicat`이라는 이름으로 실행 파일이 만들어 집니다. 현재 이 프로젝트는 실행 파일을 만들 필요는 없지만, 현재는 테스트를 하기 위해서 `main.rs`이 존재하기 때문에 실행 파일을 만들 수 있습니다.

## 코드 작성에서 유의할 점

코딩 스타일을 맞추기 위해서 코드를 올리기 전에 다음 명령어를 이용하여 코드를 정리하여 올립니다.

```console
cargo fmt
```

그런 다음 `cargo clippy` 명령어로 작성하신 코드의 문제점을 파악하고 이를 수정해 주세요.

```cosole
cargo clippy
```

그리고 마지막으로 테스트를 실행한 다음 문제가 없다면 코드를 올려주세요.

```cosole
cargo test
```
