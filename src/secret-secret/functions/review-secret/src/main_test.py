import unittest
from main import *

class TestModel(unittest.TestCase): 

  def test_load_model(self):
    data = load_model("./models/giberish_model_v1.pkl")

    self.assertNotEqual(data, None, "ERR: Data should not be None")

if __name__ == "__main__":
  unittest.main()