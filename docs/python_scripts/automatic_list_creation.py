"""
    `TossiCat-core`가 변환할 수 있는 모든 토시들의 목록을 정리하는 스크립트입니다. 
    현재 변환하는 토시는 대부분 "/src/transfer.rs"에 들어 있습니다. 
    우선 여기서 토시들을 추출합니다. 수작업으로 하면 목록을 작성하면 실수가 발생하기도 
    합니다. 그래서 이렇게 파이썬 스크립트를 이용하게 되었습니다. 위 파일에 들어 있는 
    것에 추가 해야 할 토시들을 다음과 같은 파일 변수로 이 스크립트를 통해서 목록에 
    포할될 수 있도록 했습니다.

    - `Eul_variations`
    - `Ka_variations`
    - `Neun_variations`
    - `Wa_variations`

    수정한 다음, `black`으로 정리합니다.

    black  docs/python_scripts/automatic_list_creation.py

Usage:
    python docs/python_scripts/automatic_list_creation.py
"""

import os
import re
import json

temp_dir = os.path.dirname(
    os.path.abspath(os.path.dirname(os.path.abspath(os.path.dirname(__file__))))
)

file_names = os.path.abspath(temp_dir)

file_name_transfer_rs = temp_dir + "/src/transfer.rs"

if os.path.isfile(file_name_transfer_rs):
    with open(file_name_transfer_rs, encoding="utf-8") as f:
        read_transfer_rs = f.read()
else:
    print("No!")

# 변환하고 있는 토시 상수들의 리스트를 가지고 옵니다.
# 우선 "const"와 ";" 사이에 있는 것을 모두 가져옵니다.
pattern = re.compile("(?<=const )(.*?)(?=\;)")
temp_list = re.findall(pattern, read_transfer_rs)

result = []

# 상수 속
for item in temp_list:
    # "(.*?)" 다옴표만 가져오기
    pattern = re.compile('"(.*?)"')
    sub_temp = re.findall(pattern, item)

    result = result + sub_temp

# result = sorted(set(result))

# print(result)
# print(len(result))

# 아래 것들은 각 토시가 변환해서 들어 오는 모든 경우의 수를 나열한 것입니다.
# 이렇게 들어오는 것들도 모두 처리할 수 있기 때문에
# 아래 목록 모두도 변환 처리 가능 토시 목록에 넣어야 합니다.
# 아래에서도 언급하겠지만, 괄호가 들어있는 토시들은 변환 처리 가능 토시 목록에는 들어가지만,
# 공식적으로 처리 가능 목록에서는 제외할 예정입니다.

Eul_variations = ["(을)를", "(를)을", "를(을)", "을(를)"]
Ka_variations = ["(이)가", "(가)이", "이(가)", "가(이)"]
Neun_variations = ["(은)는", "(는)은", "는(은)", "은(는)"]
Wa_variations = ["(와)과", "(과)와", "과(와)", "와(과)"]

result_1 = result + Eul_variations + Ka_variations + Neun_variations + Wa_variations

result_1 = sorted(set(result_1))

# 원칙적으로 하면 1번째 항목이 원래 "ㄴ들"과 같은 형식인데
# 이런 형식의 토시들은 마지막 글자에 `ㄴ`을 추가하고 "들"과 같은 것을
# 추가하는 방식으로 토시가 붙는 단어가 변하는데
# 외국어의 문자에 `ㄴ`을 추가할 수 없기 때문에
# 이 토시 종류를 명확하게 하기 위해서 0번째에 `ㄴ`을 넣고
# 그리고 1번째 항목에 "들"과 같이 `ㄴ`을 뺀 나머지 부분을 넣었습니다.
# 따라서 0번과 1번은 토시 목록에는 들어갈 수 없는 부분입니다.
# 그래서 위 목록에서 아래 목록에 들어 있는 0번째와 1번째 토시를 제외합니다.
# 여기에 속한 토시 목록에 대한 설명은 `src/transfer.rs` 에서
# `when_last_jamo_nieun()` 함수 설명을 참고하시면 됩니다.
#
# - INDEUL_variations
# - INJEUK_variations

INDEUL_variations = ["ㄴ", "들", "인들"]

result_1.remove(INDEUL_variations[0])
result_1.remove(INDEUL_variations[1])

# 여기서는 0번째를 지우지 않는 이유는 앞에서 이미 "ㄴ"를 지웠기 때문에
# 또 지우면 에러가 발생하기 때문입니다.

INJEUK_variations = ["ㄴ", "즉", "인즉"]

result_1.remove(INJEUK_variations[1])

result_1 = sorted(result_1)

print("")
print("# 이 프로젝트에서 다루고 있는 총 토시 목록")
print("")
print("이 글은 이 프로젝트가 다룰 수 있는 토시 목록을 항상 최근 것으로 갱신하는 문서입니다. ")
print("이 프로젝트가 처리할 수 있는 토시에는 다음과 같이 2가지 종류가 있습니다.")
print("")
print("- 붙일 단어에 따라 변환하는 토시들")
print("- 붙일 단어에 따라 변환할 필요가 없는 토시들")
print("")
print("이 두 종류를 ")
print("")
print("## 붙일 단어에 따라 변환하는 토시 목록")
print("")
print("이 목록은 `/src/transfer.rs`에서 뽑을 수 있습니다. ")
print("이 파일에서 현재 이 라이브러리에 하는 일 중 가장 중요한 변환할 토시를 확인하고 변환하는 일을 합니다. ")
print("아래 목록은 현재 이 라이브러리에서 처리할 수 있는 단어에 따라 변환할 수 있는 토시 목록입니다. ")
print("")
print(f"- 갯수: {len(result_1)}")
print("")
# print(result_1)
print("```json")
print(json.dumps(result_1, ensure_ascii=False, sort_keys=True, indent=4))
print("```")
print("")

# 아래 코드는 괄호가 들어 있는 것과 아닌 것을 구분해 분류하기 위한 것입니다.
parenthesis_exist_in_result = []
parenthesis_is_not_exist_in_result = []

for item in result_1:
    if item.find("(") != -1:
        parenthesis_exist_in_result.append(item)
    else:
        parenthesis_is_not_exist_in_result.append(item)

print("## 변환을 해서 처리할 수 있는 목록 중에서 괄호가 들어 있는 토시들")
print("")
print("이 프로젝트에서는 현재 입력된 외국어 단어를 현지 외국어 발음으로 읽어 낼 수 없습니다. ")
print("그렇다보니 외국어 단어에는 토시를 그 단어의 발음에 맞게 변환할 수 없습니다. ")
print("이런 경우에는 '(이)가'과 같이 괄호를 이용해 변화할 토시를 병기해서 반환하고 있습니다. ")
print("그리고 이렇게 처리하고 있는 토시들을 우리가 처리할 수 있는 토시 목록에 넣지 않고 있습니다. ")
print("이렇게 처리하고 있는 토시들을 우리가 처리할 수 있는 토시 목록에 넣지 않고 있습니다. ")
print("왜냐하면 내부적으로는 이런 토시들도 처리하고 있지만,")
print("굳이 외부에 공개할 필요가 없다고 생각하고 있기 때문입니다. ")
print("")
print(f"- 갯수: {len(parenthesis_exist_in_result)}")
print("")
print("```json")
print(
    json.dumps(
        parenthesis_exist_in_result, ensure_ascii=False, sort_keys=True, indent=4
    )
)
print("```")
print("")

print("## 변환을 해서 처리할 수 있는 목록 중에서 괄호가 없는 토시들")
print("")
print("이 토시들이 이 프로젝트에서 공식적으로 처리할 수 있는 붙일 단어에 따라 변하는 토시 목록입니다. ")
print("처리하 위에 괄호 있는 것들까지 포함해서 대표할 수 있는 토시들 입니다")
print("이걸 외부적으로 공개해야 합니다. ")
print("")
print(f"- 갯수: {len(parenthesis_is_not_exist_in_result)}")
print("")
print("```json")
print(
    json.dumps(
        parenthesis_is_not_exist_in_result, ensure_ascii=False, sort_keys=True, indent=4
    )
)
print("```")
print("")

not_need_to_be_converted_tossi_list = {
    "에게로",
    "께",
    "조차",
    "게",
    "대로",
    "마다",
    "께서",
    "뿐",
    "거나",
    "한테서",
    "보다",
    "부터",
    "에다가",
    "에게",
    "이다",
    "까지",
    "마저",
    "한테",
    "에서",
    "에서부터",
    "만",
    "커녕",
    "같이",
    "에",
    "에게서",
    "처럼",
    "하고",
    "의",
    "밖에",
    "마냥",
    "도",
}

# 현재 이 라이브러리가 처리할 수 있는지 없는지 판단하는 목록이
# verifier.rs 에 `TOSSI_LIST`이라는 이름으로 들어 있습니다.
# 이것을 봅아내면 현재 처리할 수 있는 전체 목록이 나오게 됩니다.
file_name_transfer_rs = temp_dir + "/src/verifier.rs"

if os.path.isfile(file_name_transfer_rs):
    with open(file_name_transfer_rs, encoding="utf-8") as f:
        read_verifier_rs = f.read()
else:
    print("No!")

# verifier.rs 코드에서 엔터 문자를 제거한다.
read_verifier_rs = read_verifier_rs.replace("\n", "")
# print(f"full: {read_verifier_rs}")
# 앞의 `const TOSSI_LIST` 을 뽑기 위해 `];`으로 자른다.
# 자르고 나면 맨 처음 부분 `[0]`에 우리가 뽑을 것이 들어가게 된다.
# 참고로 이렇게 자르면 `]`만 없어지기 때문에, 남은 '['을 제거해야 한다.
temp_str = read_verifier_rs.split("];")
# `const TOSSI_LIST` 내용만 필요하기 때문에 `=`으로 자른다.
# 이번에는 뒷 부분만 필요하다.
temp_str = temp_str[0].split("=")
temp_str = temp_str[1].replace("[", "")
# 보기 편하게 스페이스도 제거한다.
temp_str = temp_str.replace(" ", "")
# `split`을 이용해 list 만들기 위해 `"`을 제거한다.
temp_str = temp_str.replace('"', "")
# list로 만든다.
total_list = temp_str.split(",")
# temp_str 문자열 마지막이 `, '']``으로 되어 있어 빈 원소가 있다.
# 아래 print를 참고하자.
# print(f"test: {temp_list}")
# filter를 이용해 제거한다. 다음 링크 참고하자.
# https://stackoverflow.com/questions/3845423/remove-empty-strings-from-a-list-of-strings
total_list = list(filter(None, total_list))

not_need_to_be_converted_tossi_list = set(total_list) - set(parenthesis_exist_in_result)

not_need_to_be_converted_tossi_list = sorted(not_need_to_be_converted_tossi_list)

print("변환이 필요 없는 토시 목록 입니다. ")
print("이것들도 모두 이 라이브러리로 처리할 수 있는 것입니다. ")
print("왜냐하면 이 라이브러리가 변환할 필요가 없다는 것을 알려주기 때문입니다. ")
print("변환이 필요 없는 토시 목록은 라이브러리에서")
print("처리할 수 있는 토시 총 목록. `/src/verifier.rs`에서 뽑아 냈습니다. ")
print("")
print(f"갯수는: {len(not_need_to_be_converted_tossi_list)}")
print("")
print(not_need_to_be_converted_tossi_list)
print("")

print("이 라이브러리가 처리할 수 있는 총 토시 목록")
print("변환이 필요 없는 토시 목록 + 이 라이브러리가 처리할 수 있는 토시 총 목록")
print("이 목록이 내부적으로 처리하고 있는 토시 목록")
print("왜냐하면 괄호가 다양하게 들어가 같은 토시인데도 중복된 것들을 처리하고 있기 때문입니다. ")
total_result = result_1 + not_need_to_be_converted_tossi_list
print("")
print(f"갯수는: {len(total_result)}")
print("")
# print(total_result)
print(json.dumps(total_result, ensure_ascii=False))
print("")

print("명시적으로 이 라이브러리가 처리할 수 있는 총 토시 목록")
print("변환이 필요 없는 토시 목록 + 이 라이브러리가 처리할 수 있는 토시 총 목록")
print("이 목록이 대외적으로 발표해야 하는 처리할 수 있는 토시 목록")
print("괄호가 다양하게 들어가 중복된 것들을 제거한 것이 이것입니다. ")
temp_result = not_need_to_be_converted_tossi_list + parenthesis_is_not_exist_in_result
print("")
print(f"갯수는: {len(temp_result)}")
print("")
# print(total_result)
print(json.dumps(temp_result, ensure_ascii=False))
print("")

# 아래 코드는 `docs/total_tossi.json`이라는 이름으로 
# 뽑아낸 목록을 저장하는 코드 입니다.

total = {}

total["전체_목록"] = temp_result
total["괄호_포함_전체_목록"]= total_result
total["변환할_필요가_없는_목록"] = not_need_to_be_converted_tossi_list
total["변환할_필요가_있는_목록"] = parenthesis_is_not_exist_in_result

with open("docs/total_tossi.json", "w") as outfile:
    json.dump(total, outfile, ensure_ascii=False, indent=4)
