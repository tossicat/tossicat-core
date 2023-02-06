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

# 변환하고 있는 상수들의 리스트를 가지고 온다.
# "const"와 ";" 사이에 있는 것을 모두 가져온다.
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

Eul_variations = ["(을)를", "(를)을", "를(을)", "을(를)"]
Ka_variations = ["(이)가", "(가)이", "이(가)", "가(이)"]
Neun_variations = ["(은)는", "(는)은", "는(은)", "은(는)"]
Wa_variations = ["(와)과", "(과)와", "과(와)", "와(과)"]

result_1 = result + Eul_variations + Ka_variations + Neun_variations + Wa_variations

result_1 = sorted(set(result_1))

# 위 목록에서 아래 목록에 들어 있는 0번째와 1번째 토시를 제외합니다.
# 왜냐하면 이 토시들은 외국어의 문자에 `ㄴ`을 추가할 수 없기 때문에
# 이 토시 종류를 명확하게 하기 위해서 0번째에 `ㄴ`을 넣었습니다.
# 그리고 1번째 토시는 원래 "ㄴ들"인데 이것 또한 앞에서

INDEUL_variations = ["ㄴ", "들", "인들"]

result_1.remove(INDEUL_variations[0])
result_1.remove(INDEUL_variations[1])


result_1 = sorted(result_1)

print("단어에 따라 변환하는 토시 중에서 이 라이브러리에서")
print("처리할 수 있는 토시 총 목록. `/src/transfer.rs`에서 뽑아 냈습니다.")
print("")
print(f"갯수는: {len(result_1)}")
print("")
# print(result_1)
print(json.dumps(result_1, ensure_ascii=False))
print("")

parenthesis_exist_in_result = []
parenthesis_is_not_exist_in_result = []

for item in result_1:
    if item.find("(") != -1:
        parenthesis_exist_in_result.append(item)
    else:
        parenthesis_is_not_exist_in_result.append(item)

print("변환을 해서 처리할 수 있는 목록 중에서 괄호가 들어 있는 토시들")
print("이것들은 굳이 외부에 공개할 필요가 없습니다.")
print("")
print(f"갯수는: {len(parenthesis_exist_in_result)}")
print("")
print(parenthesis_exist_in_result)
print("")

print("변환을 해서 처리할 수 있는 목록 중에서 괄호가 없는 토시들")
print("이것들은 위에 괄호 있는 것들까지 포함해서 대표할 수 있는 토시들 입니다")
print("이걸 외부적으로 공개해야 합니다.")
print("")
print(f"갯수는: {len(parenthesis_is_not_exist_in_result)}")
print("")
print(parenthesis_is_not_exist_in_result)
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

not_need_to_be_converted_tossi_list = sorted(not_need_to_be_converted_tossi_list)

print("변환이 필요 없는 토시 목록 입니다.")
print("이것들도 모두 이 라이브러리로 처리할 수 있는 것입니다.")
print("왜냐하면 이 라이브러리가 변환할 필요가 없다는 것을 알려주기 때문입니다.")
print("")
print(f"갯수는: {len(not_need_to_be_converted_tossi_list)}")
print("")
print(not_need_to_be_converted_tossi_list)
print("")

print("이 라이브러리가 처리할 수 있는 총 토시 목록")
print("변환이 필요 없는 토시 목록 + 이 라이브러리가 처리할 수 있는 토시 총 목록")
print("이 목록이 내부적으로 처리하고 있는 토시 목록")
print("왜냐하면 괄호가 다양하게 들어가 같은 토시인데도 중복된 것들을 처리하고 있기 때문입니다.")
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
print("괄호가 다양하게 들어가 중복된 것들을 제거한 것이 이것입니다.")
temp_result = not_need_to_be_converted_tossi_list + parenthesis_is_not_exist_in_result
print("")
print(f"갯수는: {len(temp_result)}")
print("")
# print(total_result)
print(json.dumps(temp_result, ensure_ascii=False))
print("")
