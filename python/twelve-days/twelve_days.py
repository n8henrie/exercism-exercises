gifts = [
    "a Partridge in a Pear Tree.",
    "two Turtle Doves, and",
    "three French Hens,",
    "four Calling Birds,",
    "five Gold Rings,",
    "six Geese-a-Laying,",
    "seven Swans-a-Swimming,",
    "eight Maids-a-Milking,",
    "nine Ladies Dancing,",
    "ten Lords-a-Leaping,",
    "eleven Pipers Piping,",
    "twelve Drummers Drumming,",
]

ordinals = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
]


def verse(idx):
    return (
        f"On the {ordinals[idx]} day of Christmas my true love gave to me: "
        f"{' '.join(gifts[idx] for idx in range(idx, -1, -1))}"
    )


def recite(start_verse, end_verse):
    return [verse(idx) for idx in range(start_verse - 1, end_verse)]
