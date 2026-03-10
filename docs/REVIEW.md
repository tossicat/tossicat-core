# TossiCat Core 프로젝트 리뷰

## 1. 프로젝트 개요

- **프로젝트명**: TossiCat Core
- **버전**: 0.7.0 (Cargo.toml 기준, RELEASES.md에 0.8.0 작업 내역 존재)
- **목적**: 입력된 한글 단어에 맞춰 조사(토시)를 자동으로 변환하는 Rust 라이브러리
- **라이센스**: MIT

### 핵심 기능

- 단일 단어-토시 쌍의 변환 (`postfix()`)
- 복수의 단어-토시 쌍을 포함한 문장 일괄 변환 (`modify_sentence()`)
- 변형된 토시 추출 (`transform()`)
- 한글 음절 처리 (초성·중성·종성 분리/결합)
- 숫자를 한글로 변환
- 총 192개의 토시 처리 가능 (변환 필요 토시 159개 + 불변 토시 33개)

---

## 2. 프로젝트 구조

### 소스 파일

| 파일 | 라인수 | 역할 |
|------|--------|------|
| lib.rs | 302 | 공개 API 정의, 모듈 통합 |
| error.rs | 126 | 에러 타입 정의 (ValueError, ParseError) |
| hangeul.rs | 172 | 한글 음절 처리 (분리/결합, 종성 변경) |
| filter.rs | 185 | 단어 필터링 (마지막 글자 추출, 종성 판단) |
| bracket.rs | 322 | 중괄호 파싱 및 단어-토시 추출 |
| number.rs | 165 | 숫자를 한글 발음으로 변환 |
| identifier.rs | 274 | 토시 종류 판별 및 변환 방식 결정 |
| tossi.rs | 259 | 처리 가능한 토시 목록 정의 (192개) |
| transfer.rs | 1241 | 토시 변환 로직 (가장 큰 모듈) |
| verifier.rs | 97 | 입력값 검증 (토시 유효성, 단어 길이) |

**총 소스코드**: 약 3143 라인

### 테스트 파일

| 파일 | 테스트 수 | 내용 |
|------|-----------|------|
| tests/lib.rs | 19 | `modify_sentence`, `postfix`, `transform` 통합 테스트 |
| tests/bracket.rs | 7 | 괄호 파싱 및 에러 케이스 테스트 |
| tests/hangeul.rs | 3 | 한글 음절 처리 테스트 |
| tests/filter.rs | 1 | 문자 필터링 테스트 |
| tests/number.rs | 1 | 숫자 변환 테스트 |
| src 내부 테스트 | 34 | 내부 모듈별 유닛 테스트 |
| Doc 테스트 | 21 | 문서에 포함된 예제 테스트 (2개 무시) |

**총 테스트 수**: 86개 (모두 통과, 2개 무시)

### 문서 파일

| 파일 | 내용 |
|------|------|
| docs/terms.md | 한글/영어 용어 정의, 토시 영어명 |
| docs/errors.md | 에러 타입 및 에러 코드 정의 |
| docs/available_tossi_list.md | 처리 가능한 모든 토시 목록 |
| docs/unavailable_tossi_list.md | 미지원 토시 목록 및 완료 이력 |
| docs/total_tossi.json | JSON 형식의 전체 토시 목록 |
| docs/HOW_TO_ADD_TOSSI.md | 새로운 토시 추가 가이드 (10단계) |
| docs/REVIEW.md | 프로젝트 리뷰 (이 문서) |

---

## 3. Public API

### 핵심 함수

| 함수 | 시그니처 | 역할 |
|------|----------|------|
| `modify_sentence` | `(&str) -> Result<String, ParseError>` | 문장 내 복수 토시 일괄 변환 |
| `postfix` | `(&str, &str) -> Result<String, ValueError>` | 단어에 토시를 붙여 반환 |
| `transform` | `(&str, &str) -> Result<(String, String), ValueError>` | 단어와 토시를 분리해 반환 |

### 한글 음절 처리 함수

| 함수 | 시그니처 | 역할 |
|------|----------|------|
| `split_phonemes` | `(char) -> Result<[char; 3], ValueError>` | 한글 음절을 초·중·종성으로 분리 |
| `join_phonemes` | `([char; 3]) -> char` | 초·중·종성을 한글 음절로 결합 |
| `modify_final_jamo` | `(char, char) -> char` | 한글 글자의 종성 변경 |

### 필터링/검사 함수

| 함수 | 시그니처 | 역할 |
|------|----------|------|
| `find_last_letter` | `(&str) -> char` | 문자열의 마지막 한글/숫자 글자 반환 |
| `guess_final_letter` | `(&str) -> char` | 마지막 글자의 종성 반환 |
| `change_num_to_hangeul` | `(&str) -> String` | 숫자를 한글 발음으로 변환 |

---

## 4. 모듈별 분석

### hangeul (한글 처리)

- 유니코드 기반 변환: `'가'`(U+AC00)를 기준으로 오프셋 계산
- 초성 19개, 중성 21개, 종성 28개 배열 상수 정의
- `split_phonemes`는 비한글 입력 시 `Err(ValueErrorType::InvalidCharacter)` 반환

### filter (필터링)

- 괄호 내용 제거, 숫자를 한글로 변환 후 마지막 글자 추출
- `hangeul`, `number` 모듈 의존

### transfer (토시 변환 로직)

- 1241 라인으로 전체 코드의 약 39% 차지
- 50개 토시별 변환 상수 정의 (3-튜플: 병기, 무받침, 유받침)
- 6가지 변환 방식: Blank, RiEulAndBlank, OnlyKa, LastJamoNieun, LastJamoRieul, Nothing

### identifier (토시 분류)

- `TossiKind` enum: 50개 토시 종류 + Others
- `TransTossiWhen` enum: 6가지 변환 방식
- 1~4글자 토시를 길이별로 분류

### bracket (괄호 파싱)

- 스택 기반 괄호 쌍 매칭
- 중첩 감지 및 6가지 에러 케이스 처리

### verifier (입력 검증)

- 토시 유효성 검사 (TOSSI_LIST 기반)
- 단어 길이 제한 (50자)

---

## 5. 에러 처리

### ValueError (값 에러)

| 에러 코드 | 이름 | 설명 |
|-----------|------|------|
| 101 | InvalidTossi | 올바른 토시가 아닙니다 |
| 102 | InvalidCharacter | 올바른 한글 문자가 아닙니다 |
| 103 | LimitLength | 단어 길이가 50을 초과합니다 |

### ParseError (파싱 에러)

| 에러 코드 | 이름 | 설명 |
|-----------|------|------|
| 201:1XX | InvalidValue | ValueError를 래핑하여 반환 |
| 203 | AreNotBalanced | 중괄호 개수가 올바르지 않습니다 |
| 204 | IsNotBrace | 괄호가 중괄호가 아닙니다 |
| 205 | NestedParentheses | 중복 중괄호가 있습니다 |
| 206 | SplitTossiWord | 토시와 단어가 쉼표로 나눠져 있지 않습니다 |
| 207 | TossiIsEmpty | 토시가 비어 있습니다 |
| 208 | WordIsEmpty | 단어가 비어 있습니다 |

### 에러 처리 흐름

```text
postfix() / transform()
  └─ verifier::verify_value()
       ├─ InvalidTossi
       └─ LimitLength

split_phonemes()
  └─ InvalidCharacter

modify_sentence()
  └─ bracket::modify_pairs()
       ├─ AreNotBalanced
       ├─ IsNotBrace
       ├─ NestedParentheses
       ├─ SplitTossiWord
       ├─ TossiIsEmpty
       └─ WordIsEmpty
  └─ postfix() → ValueError → ParseError::InvalidValue()
```

---

## 6. 코드 품질

### Clippy 결과

- 프로덕션 코드: 경고 없음
- 테스트 코드: 경고 없음

### Unwrap 사용 현황 (5개)

모든 `unwrap()` 호출이 사전 검증된 조건에서 사용되므로 안전합니다.

| 위치 | 안전성 근거 |
|------|------------|
| hangeul.rs (라인 77-79) | `is_hangul_syllable()` 사전 검증 |
| hangeul.rs (라인 83) | 유효한 유니코드 범위 내 계산 |
| hangeul.rs (라인 103) | `is_hangeul()` 사전 검증 |

### 의존성

- 외부 크레이트 의존 없음 (순수 Rust 표준 라이브러리만 사용)

---

## 7. 테스트 현황

### 통계

- **총 테스트**: 86개
- **통과**: 86개 (100%)
- **무시**: 2개 (doc-test)

### 커버리지 영역

- 핵심 기능: `postfix`, `modify_sentence`, `transform` (정상 + 에러 경로)
- 한글 처리: 음절 분해/결합, 종성 변경, 비한글 에러 처리
- 숫자 변환: 자리수 단위, 관용 표현
- 괄호 파싱: 정상 케이스 + 6가지 에러 케이스
- 에러 처리: 유효성 검사, 길이 제한, 잘못된 토시
- 특수 케이스: 외국어 병기, 대명사 변환 (누구→누가)
- RiEulAndBlank 패턴: 로도/로만/로는/로의 (ㄹ 받침 특수 처리)

---

## 8. 내부 의존성 그래프

```text
lib.rs
  ├─ bracket (공개)
  ├─ error
  ├─ filter → hangeul, number
  ├─ hangeul → error
  ├─ identifier → filter
  ├─ transfer → identifier, hangeul, filter
  └─ verifier → tossi
```

---

## 9. 종합 평가

| 항목 | 평가 |
|------|------|
| 코드 품질 | 우수 (clippy 경고 없음, 안전한 unwrap 사용) |
| 테스트 | 우수 (86개 테스트, 100% 통과) |
| 문서화 | 우수 (모듈/함수 독스트링, 용어집, 에러 문서, 토시 추가 가이드) |
| 의존성 | 최소 (외부 의존 없음) |
| 기능 범위 | 포괄적 (192개 토시, 50종 변환, 특수 케이스 처리) |
| 에러 처리 | 체계적 (에러 코드 기반, Result 반환) |

프로덕션 사용에 적합한 상태입니다.
