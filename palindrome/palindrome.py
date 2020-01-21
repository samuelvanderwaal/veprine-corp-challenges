def is_palindrome(sequence: str) -> bool:
    return ''.join(c for c in sequence if c.isalnum()) == ''.join(c for c in sequence if c.isalnum())[::-1]


def is_palindrome2(word: str) -> bool:
    if len(word) <= 1:
        return True

    if word[0] != word[-1]:
        return False

    word = word[1:-1]
    return is_palindrome(word)


if __name__ == "__main__":
    assert is_palindrome("kayak")
    assert is_palindrome("ss")
    assert is_palindrome("s")
    assert is_palindrome("")
    assert is_palindrome("toot")
    assert is_palindrome("racecar")
    assert not is_palindrome("word")
    assert not is_palindrome("toots")
    assert not is_palindrome("streerp")
    assert not is_palindrome("ats")
    assert not is_palindrome("at")

    assert not is_palindrome2("word")
    assert not is_palindrome2("toots")
    assert not is_palindrome2("streerp")
    assert not is_palindrome2("ats")
    assert not is_palindrome2("at")
    assert is_palindrome2("ss")
    assert is_palindrome2("s")
    assert is_palindrome2("")
    assert is_palindrome2("toot")
    assert is_palindrome2("kayak")
    assert is_palindrome2("racecar")
