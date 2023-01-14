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

# 2차
# 아래 코드는

temp_dir = os.path.dirname(
    os.path.abspath(os.path.dirname(os.path.abspath(os.path.dirname(__file__))))
)

file_names = os.path.abspath(temp_dir)

file_name_transfer_rs = temp_dir + "/src/verifier.rs"

if os.path.isfile(file_name_transfer_rs):
    with open(file_name_transfer_rs, encoding="utf-8") as f:
        read_transfer_rs = f.read()
else:
    print("No!")

read_transfer_rs = read_transfer_rs.replace("\n", " ")
read_transfer_rs = read_transfer_rs.replace("     ", "")
read_transfer_rs = read_transfer_rs.replace(";", ";\n")

# 대 괄호를 뽑아준다.
p = re.compile("\[(.+)\](.+)")
temp_list = re.findall(p, read_transfer_rs)
# 첫번째 것에서 첫 번째 것을 뽑는다.
temp_result = temp_list[0][0]
temp_result = eval(temp_result)
last_temp = set(temp_result) - set(result)

temp_2 = {
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

last_temp = last_temp | temp_2

print(sorted(last_temp))
print(len(sorted(last_temp)))

## 전체 처리 목록 만들기

total = result_1 or list(last_temp)


print(sorted(total))
print(len(sorted(total)))
