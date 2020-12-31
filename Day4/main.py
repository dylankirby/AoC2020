import re

"""
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in: If cm, the number must be at least 150 and at most 193. If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.

"""

def validate_height(value):
    value, unit = int(value[:-2]), value[-2:]
    if unit == "cm":
        return 150 <= value <= 193
    elif unit == "in":
        return 50 <= value <= 76


def validate_hair_color(color):
    return bool(re.match("^#[0-9a-f]{6}$", color))

def validate_eye_color(color):
    return color in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

def validate_passport_id(_id):
    return (len(_id) == 9 and _id.isdigit())


VALIDATE = True
required_fields = {
    "byr": lambda x: (1920 <= int(x) <= 2002) and len(x) == 4,
    "iyr": lambda x: len(x) == 4 and (2010 <= int(x) <= 2020),
    "eyr": lambda x: len(x) == 4 and (2020 <= int(x) <= 2030),
    "hgt": lambda x: validate_height(x),
    "hcl": lambda x: validate_hair_color(x),
    "ecl": lambda x: validate_eye_color(x),
    "pid": lambda x: validate_passport_id(x),
    # "cid": lambda _: True,
}

def main():
    count = 0
    with open('data.txt') as fr:
        passports = fr.read().split("\n\n")
        num_passports = len(passports)
        print(f"Found {num_passports} passports")
        for passport in passports:
            split = re.split(r'\s|\n', passport)
            as_dict = {e.split(":")[0]: e.split(":")[1] for e in split}
            for field_name, field_validator in required_fields.items():
                value = as_dict.get(field_name, None)
                if not value:
                    break
                if VALIDATE and not field_validator(value):
                    break
            else:
                count += 1
    
    print(count)
if __name__ == "__main__":
    main()
