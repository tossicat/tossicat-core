# TossiCat core

이 프로젝트는 [tossi](https://github.com/what-studio/tossi)에서 영감을 받았습니다. 파이썬으로 구현된 앞의 **토씨 라이브러리**처럼 이 프로젝트도 임의의 단어와 그 단어에 붙일 조사(즉 토시)를 입력하면, 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔 반환해 줍니다.

왜 바꿔서 토시를 붙여야 할까요? 예를 들어 봅시다.'사과'라는 단어에 '을'이라는 조사를 붙일 경우, '을'을 붙이면 안 되고, 같은 의미를 가진 '를'을 붙여야 합니다. 왜냐하면 '을'이라는 조사는 이 조시를 붙일 단어의 마지막 단어의 밭침이 없으면 붙일 수 없기 때문입니다. 물론 '사과'라는 단어에 '처럼'이라는 조사를 붙일 경우에는 이런 변화를 고려할 필요가 없습니다. 왜냐하면 이 조사는 어떠한 단어에도 붙일 수 있기 때문입니다.

한국어를 사용하는 한국인 입장에서는 학교 교육을 받으면서 이런 것들을 자동적으로 구별할 수 있습니다. 그러나 외국인 입장에서 보면 그리 쉽지 않습니다. 그러나 더 중요한 문제는 앱 안에서 문장을 조합해서 사용자에게 표현하고자 할 때, 이런 특정 단어에 토시를 붙이는 것을 자동화하기 위해서는 이것을 자동화하는 라이브러리가 필요하게 됩니다. 바로 이 프로젝트가 이런 경우에 필요한 것입니다.

이 프로젝트는 Rust로 작성하고 있습니다. 자세한 내용은 아래 내용을 참고하세요. 참고로 이 프로젝트는 [LOPES-HUFS-tossi_for_rust](https://github.com/LOPES-HUFS/tossi_for_rust)을 확장하고자 [TossiCat](https://github.com/tossicat)이라는 조직(organization)을 만들고 목적에 맞게 코드를 각각의 저장소들(repositories)로 나눠 코딩하고자 원 코드를 옮겨서 재시작하게 되었습니다.

## 구현 함수

여기에서는 다음과 같은 2가지 기능을 지원합니다.

- `pick(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 반환합니다.
- `postfix(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 덧붙여 반환합니다.

## 이 프로젝트에서 처리 가능한 토시(조사) 목록

이 프로젝트는 두 종류의 토시가 있습니다.

1. 변환을 고려해야 하는 토시들
2. 변환할 필요가 없는 토시들

"변환할 필요가 없는 토시들"은 어떤 단어에 붙여서 문제가 되지 않습니다. 그러나 "변환 가능한 토시들"은 이 토시들이 어떤 단어에 붙일 것인가에 따라 다른 변환해야 합니다. 이 프로젝트는 이 두 종류의 토시를 다 처리할 수 있기 때문에, 어떤 토시를 변환해야 하는지, 어떤 토시는 변환할 필요가 없는 것인지를 판단해서 적절하게 변환해 줍니다.

### 변환을 고려해야 하는 토시들

현재 이 프로젝트에서 토시(tossi)는 다음과 같습니다. 지속적으로 추가할 예정입니다.

- 을 / 를
- 다 / 이다
- 이 / 가
- 은 / 는
- 나 / 이나
- 이나마/ 나마
- 이랑 / 랑
- 이란 / 란
- 이며 / 며
- 이야말로 / 야말로
- 이고 / 고
- 로 / 으로
- 로서 / 으로서
- 로써 / 으로써
- 로부터 / 으로부터

### 변환할 필요가 없는 토시들

'같이', '거나', '게', '까지', '께', '께서', '대로', '도', '마냥', '마다', '마저', '만', '밖에', '보다', '부터', '뿐', '에', '에게', '에게로', '에게서', '에다가', '에서', '에서부터', '의', '이다', '조차', '처럼', '커녕', '하고', '한테', '한테서'

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
