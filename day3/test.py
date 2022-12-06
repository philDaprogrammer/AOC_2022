f = open("input.txt", "r")
rawInput = f.read()

packs = rawInput.split("\n")

sum_priorities = 0

def get_value(item_char):
    value = 0
    if item_char.lower() == item_char:
        value = ord(item_char) - 96
    else:
        value = ord(item_char) - 38

    print(value)
    return value

# ----- part 1 ------

for pack in packs:
    totalNumberOfItems = len(pack)
    middleIndex = int(totalNumberOfItems / 2)
    seen_items = {}
    for idx, item in enumerate(pack):
        if idx < middleIndex:
            seen_items[item] = 1
        else:
            if item in seen_items:
                sum_priorities = sum_priorities + get_value(item)
                break

print(sum_priorities)
