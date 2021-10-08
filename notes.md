Case conversion in steps
1. determine word boundaries
2. construct words
3. mutate words (letter casing)
4. join words

ccase
add option to treat each "word" as a muliword token
i.e. instead of word-one word-two -t snake = word_one_word_two
it is word_one word_two

String
+ Boundaries
-> Words
+ Delim + Pattern
-> Multiword Identifier

Cases are just aliases for Delim+Patterns (most of the time)
if not some other transformation (alternating)
Cases can also be mapped to boundaries
