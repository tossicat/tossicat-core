# TossiCat core

이 프로젝트는 [tossi](https://github.com/what-studio/tossi)에서 영감을 받았습니다. 파이썬으로 구현된 앞의 **토씨 라이브러리**처럼 이 프로젝트도 임의의 단어와 그 단어에 붙일 조사를 입력하면, 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔 반환해 줍니다. 이 프로젝트는 Rust로 작성하고 있습니다. 자세한 내용은 아래 내용을 참고하세요. 참고로 이 프로젝트는 [LOPES-HUFS-tossi_for_rust](https://github.com/LOPES-HUFS/tossi_for_rust)을 확장하고자 [TossiCat](https://github.com/tossicat)이라는 조직(organization)을 만들고 목적에 맞게 코드를 각각의 저장소들(repositories)로 나눠 코딩하고자 원 코드를 옮겨서 재시작된 것입니다.

## 구현 함수

여기에서는 다음과 같은 2가지 기능을 지원합니다.

- `pick(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 반환합니다.
- `postfix(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 덧붙여 반환합니다.

## 프로젝트에서 변환 가능한 토시 목록

현재 이 프로젝트에서  토시(tossi)는 다음과 같습니다. 지속적으로 추가할 예정입니다.

- 은 / 는
- 이 / 가
- 으로 / 로
- 다 / 이다
- 을 / 를
- 로 / 으로
- 로서 / 으로서
- 로써 / 으로써
- 로부터 / 으로부터
- 나 / 이나
- 이랑 / 랑
- 이란 / 란

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
