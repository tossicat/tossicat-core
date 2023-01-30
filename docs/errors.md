# TossiCat Error

|error type|error code|error name|description|
|----|----|-----|-----|
|ValueError|101|InvalidTossi|올바른 토씨가 아닙니다. <br/> This value is not correct tossi|
|ValueError|102|LimitLength|단어 길이가 50를 초과합니다. <br/> The length has been exceeded. Set the word length to less than 50|
|ParseError|201:101|InvalidValue(InvalidTossi)|파싱된 토씨가 올바르지 않습니다. ValueError를 반환합니다. <br/> Return to ValueError|
|ParseError|201:102|InvalidValue(LimitLength)|파싱된 단어 길이가 50을 초과합니다. ValueError를 반환합니다. <br/> Return to ValueError|
|ParseError|203|AreNotBalanced| 중괄호 개수가 올바르지 않습니다. <br/> The sentence can not be parsed. Please check the sentence has incorrect parentheses.|
|ParseError|204|IsNotBrace| 괄호가 중괄호가 아닙니다. <br/> The sentence can not be parsed. Please set the parentheses as a brace.|
|ParseError|205|NestedParentheses| 중복 중괄호가 있습니다. <br/> The sentence includes Nested Parentheses.|
|ParseError|206|SplitTossiWord| 토씨와 단어가 쉼표(,)로 나눠져있지 않습니다. <br/> The sentence includes Nested Parentheses. Please separate words and tossi with a comma.|
|ParseError|207|TossiIsEmpty| 토씨가 비어있어 파싱할 수 없습니다. <br/> The sentence can not be parsed. Please fill the tossi section in the parentheses.|
|ParseError|208|WordIsEmpty| 단어가 비어있어 파싱할 수 없습니다. <br/> The sentence can not be parsed. Please fill the word section in the parentheses.|
