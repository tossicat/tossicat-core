# 새로운 토시 추가 가이드

이 문서는 tossicat-core에 새로운 토시를 추가하는 코딩 순서를 설명합니다.

---

## 사전 준비

새로운 토시를 추가하기 전에 먼저 아래 사항을 결정해야 합니다.

1. **변환이 필요한 토시인가?**
   - 받침 유무에 따라 형태가 바뀌는 토시 (예: 은/는, 을/를) → 변환 필요
   - 형태가 바뀌지 않는 토시 (예: 까지, 만큼, 처럼) → 변환 불필요

2. **어떤 변환 방식인가?** (변환이 필요한 경우)
   - `Blank`: 받침 유무에 따라 변환 (가장 일반적, 예: 은/는)
   - `RiEulAndBlank`: ㄹ 받침을 받침 없는 것으로 취급 (예: 로/으로)
   - `OnlyKa`: 단어 자체가 변하는 특수 케이스 (예: 누구→누가)
   - `LastJamoNieun`: 받침 없는 글자에 ㄴ 받침 추가 (예: 인들)
   - `LastJamoRieul`: 받침 없는 글자에 ㄹ 받침 추가 (예: 일랑)

3. **토시의 세 가지 형태 파악**
   - 병기 형태: 외국어 단어 뒤에 붙는 형태 (예: `"(은)는"`)
   - 받침 없을 때: (예: `"는"`)
   - 받침 있을 때: (예: `"은"`)

---

## 추가 순서

아래 예시는 가상의 토시 `"(음)임"` / `"임"` / `"음"`을 추가하는 과정입니다.

### 1단계: `src/tossi.rs` — 토시 목록에 추가

`TOSSI_LIST` 배열에 새 토시의 모든 형태를 추가합니다.

```rust
// 변경 전
pub const TOSSI_LIST: [&str; 123] = [
    // ...기존 토시들...
    "일랑",
];

// 변경 후 (배열 크기도 함께 수정)
pub const TOSSI_LIST: [&str; 126] = [
    // ...기존 토시들...
    "일랑",
    "(음)임",
    "임",
    "음",
];
```

변환이 필요 없는 토시라면 `UNCHANGED_LIST`에도 추가합니다.

### 2단계: `src/identifier.rs` — `TossiKind` enum에 variant 추가

```rust
pub enum TossiKind {
    // ...기존 variant들...
    Illang,
    Im,      // 새로 추가
    Others,
}
```

### 3단계: `src/identifier.rs` — 글자 수별 분류 함수에 매칭 추가

토시의 글자 수에 따라 해당하는 함수에 매칭 arm을 추가합니다.

- 1글자 → `one_letter()`
- 2글자 → `two_letters()`
- 3글자 → `three_letters()`
- 4글자 → `four_letters()`

```rust
// 1글자 토시의 경우 one_letter()에 추가
fn one_letter(element: char) -> TossiKind {
    match element {
        // ...기존 매칭...
        '임' | '음' => TossiKind::Im,  // 새로 추가
        _ => TossiKind::Others,
    }
}
```

괄호가 포함된 형태 `"(음)임"`은 `Tossi::new()`에서 괄호 제거 후
분류하므로 별도 처리가 필요 없습니다.

### 4단계: `src/identifier.rs` — 변환 방식 매핑

`Tossi::new()` 함수 내 `temp_trans_tossi_when` 매칭에 추가합니다.
대부분의 토시는 `Blank` 방식이므로 `_ => TransTossiWhen::Blank`에 해당하여
별도 추가가 필요 없습니다.

특수한 변환 방식이 필요한 경우에만 명시적으로 추가합니다.

```rust
let temp_trans_tossi_when = match temp_kind {
    // ...기존 매칭...
    // Im은 Blank 방식이므로 _ 매칭에 해당하여 추가 불필요
    // 특수 방식이 필요하다면 아래처럼 명시적으로 추가
    // TossiKind::Im => TransTossiWhen::RiEulAndBlank,
    _ => TransTossiWhen::Blank,
};
```

### 5단계: `src/transfer.rs` — 변환 상수 추가

세 가지 형태를 튜플로 정의합니다.
순서: `(병기 형태, 받침 없을 때, 받침 있을 때)`

```rust
// 기존 상수들 아래에 추가
const IM: (&str, &str, &str) = ("(음)임", "임", "음");
```

### 6단계: `src/transfer.rs` — `get_variants()` 함수에 매칭 추가

```rust
fn get_variants(kind: &TossiKind) -> (&'static str, &'static str, &'static str) {
    match kind {
        // ...기존 매칭...
        TossiKind::Im => IM,  // 새로 추가
        TossiKind::Others => (" ", " ", " "),
    }
}
```

### 7단계: `src/lib.rs` — `postfix()` 함수 확인

`postfix()` 함수에서 `TossiKind`에 따라 단어와 토시를 합치는 방식이
다를 수 있습니다. 대부분의 토시는 `_ =>` 기본 분기에서 처리됩니다.

단어 자체가 변형되는 특수한 경우(예: `Ka`, `Indeul`)에만
별도의 매칭 arm을 추가합니다.

```rust
// postfix() 내부
match temp.kind {
    TossiKind::Others => Ok(word.to_string() + tossi),
    // 단어가 변형되는 경우 (토시만 반환하므로 word를 더하지 않음)
    TossiKind::Ka => Ok(transfer::tossi(word, temp)),
    TossiKind::Indeul => Ok(transfer::tossi(word, temp)),
    // ...
    // 일반적인 경우 (word + 변환된 토시)
    _ => Ok(word.to_string() + &transfer::tossi(word, temp)),
}
```

### 8단계: 테스트 추가

`tests/` 디렉터리에 새 토시에 대한 테스트를 추가합니다.

```rust
// tests/lib.rs 에 추가
#[test]
fn _postfix_im() {
    let result = Ok("사과임".to_string());
    assert_eq!(result, postfix("사과", "임"));

    let result = Ok("사람음".to_string());
    assert_eq!(result, postfix("사람", "임"));

    let result = Ok("apple(음)임".to_string());
    assert_eq!(result, postfix("apple", "임"));
}
```

### 9단계: 테스트 및 검증 실행

```bash
cargo test
cargo clippy
```

### 10단계: 문서 업데이트

아래 문서 파일들을 업데이트합니다.

- `docs/available_tossi_list.md` — 해당 섹션에 새 토시 추가
- `docs/total_tossi.json` — JSON 목록에 새 토시 추가

이 두 파일은 `docs/python_scripts/automatic_list_creation.py` 스크립트를 실행하면
자동으로 생성할 수 있습니다. 이 스크립트는 `src/transfer.rs`에서 변환 상수를,
`src/tossi.rs`에서 `TOSSI_LIST`를 읽어서 문서를 생성합니다.

```bash
# available_tossi_list.md 생성
python docs/python_scripts/automatic_list_creation.py > docs/available_tossi_list.md

# total_tossi.json은 스크립트 실행 시 자동으로 docs/total_tossi.json에 저장됩니다.
```

---

## 수정 파일 체크리스트

| 순서 | 파일 | 수정 내용 |
|------|------|----------|
| 1 | `src/tossi.rs` | `TOSSI_LIST`에 토시 형태 추가, 배열 크기 수정 |
| 2 | `src/identifier.rs` | `TossiKind` enum에 variant 추가 |
| 3 | `src/identifier.rs` | 글자 수별 분류 함수에 매칭 arm 추가 |
| 4 | `src/identifier.rs` | `Tossi::new()`에서 변환 방식 매핑 (필요 시) |
| 5 | `src/transfer.rs` | 변환 상수 (3-튜플) 추가 |
| 6 | `src/transfer.rs` | `get_variants()` 함수에 매칭 arm 추가 |
| 7 | `src/lib.rs` | `postfix()` 함수에 매칭 arm 추가 (필요 시) |
| 8 | `tests/lib.rs` | 테스트 케이스 추가 |
| 9 | - | `cargo test` 및 `cargo clippy` 실행 |
| 10 | `docs/` | 문서 파일 업데이트 |

---

## 변환 방식별 참고 예시

### Blank (가장 일반적)

받침 유무에 따라 단순 교체합니다.

```text
"사과" + "을" → "사과를"   (받침 없음 → 를)
"사람" + "을" → "사람을"   (받침 있음 → 을)
"apple" + "을" → "apple(을)를"  (외국어 → 병기)
```

### RiEulAndBlank (ㄹ 받침 특수 처리)

ㄹ 받침을 받침 없는 것으로 취급합니다.

```text
"나라" + "로" → "나라로"   (받침 없음 → 로)
"서울" + "로" → "서울로"   (ㄹ 받침 → 로, 받침 없는 것과 동일)
"부산" + "로" → "부산으로" (ㄹ 외 받침 → 으로)
```

### LastJamoNieun (ㄴ 받침 추가)

받침 없는 글자에 ㄴ 받침을 추가한 뒤 접미사를 붙입니다.

```text
"나" + "인들" → "난들"   (나→난 + 들)
"사람" + "인들" → "사람인들" (받침 있음 → 그대로)
```

### LastJamoRieul (ㄹ 받침 추가)

받침 없는 글자에 ㄹ 받침을 추가한 뒤 접미사를 붙입니다.

```text
"나" + "일랑" → "날랑"   (나→날 + 랑)
"사람" + "일랑" → "사람일랑" (받침 있음 → 그대로)
```
