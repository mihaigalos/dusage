import unittest
import os.path
import subprocess
from enum import Enum
import pprint

allowed_difference_threshold=2.0
allowed_difference_threshold_percent=2.0

class Fields(Enum):
    Size = 0
    Used = 1
    Available = 2
    Used_percent = 3
    Mounted_on = 4

def remove_units(input):
    return input.translate({ord(i):None for i in 'BKMGTPEZYX'})

def fuzzy_compare(left, right, threshold) -> bool:
    left = float(remove_units(left))
    right = float(remove_units(right))
    result = abs(left - right) <= threshold

    if result == False:
        print(f"Threshold overstepped: {left}     {right}   by {result}")

    return result


class TestFixture(unittest.TestCase):
    def executeCommand(self, command, args):
        file = os.popen(f"{command} {args}")
        self.open_files.append(file)
        out = [s.split() for s in file.read().splitlines()]
        self.data[command] = {}
        for row in out[1:]:
            key = row[0]
            self.data[command][key] = row[1:]


    def setUp(self):
        self.open_files = []
        self.data = {}
        self.executeCommand("df", "-h")
        self.executeCommand("cargo", "run")

    def tearDown(self):
        for f in self.open_files:
            f.close()



    def test_size_same_whenTypical(self):
        for key_df, value_df in self.data["df"].items():
            if key_df in self.data["cargo"].keys():
                left = value_df[Fields.Size.value]
                right = self.data["cargo"][key_df][Fields.Size.value]
                is_below_threshold = fuzzy_compare(left, right, allowed_difference_threshold)
                self.assertTrue(is_below_threshold)

    def test_used_same_whenTypical(self):
        for key_df, value_df in self.data["df"].items():
            if key_df in self.data["cargo"].keys():
                left = value_df[Fields.Used.value]
                right = self.data["cargo"][key_df][Fields.Used.value]
                is_below_threshold = fuzzy_compare(left, right, allowed_difference_threshold)
                self.assertTrue(is_below_threshold)

    def test_available_same_whenTypical(self):
        for key_df, value_df in self.data["df"].items():
            if key_df in self.data["cargo"].keys():
                left = value_df[Fields.Available.value]
                right = self.data["cargo"][key_df][Fields.Available.value]
                is_below_threshold = fuzzy_compare(left, right, allowed_difference_threshold)
                self.assertTrue(is_below_threshold)

    def test_used_percent_same_whenTypical(self):
        for key_df, value_df in self.data["df"].items():
            if key_df in self.data["cargo"].keys():
                left = value_df[Fields.Available.value]
                right = self.data["cargo"][key_df][Fields.Available.value]
                is_below_threshold = fuzzy_compare(left, right, allowed_difference_threshold)
                self.assertTrue(is_below_threshold)

    def test_remove_units_works_when_typical(self):
        expected="100"
        actual=remove_units("100G")
        self.assertEqual(expected, actual)
    
    def test_fuzzy_compare_under_threshold_when_typical(self):
        expected = True
        actual = fuzzy_compare("100.2G", "100.7G", allowed_difference_threshold)
        self.assertEqual(expected, actual)


if __name__ == "__main__":
    unittest.main()