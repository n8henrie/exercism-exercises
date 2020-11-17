def is_isogram(string):
    seen = set()
    for letter in string.casefold():
        if letter.isalpha():
            if letter in seen:
                return False
            seen.add(letter)
    return True
