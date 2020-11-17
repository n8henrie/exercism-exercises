import string
from collections import Counter


def count_words(sentence):
    valid_chars = set(
        string.ascii_lowercase + string.digits + string.whitespace + "'"
    )
    return dict(
        Counter(
            word.strip("'")
            for word in "".join(
                letter if letter in valid_chars else " "
                for letter in sentence.lower()
            ).split()
        )
    )
