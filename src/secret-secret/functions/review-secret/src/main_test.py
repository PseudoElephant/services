import unittest
import main

class TestModel(unittest.TestCase): 

  def test_load_model(self):
    self.assertEqual(main.model_data, None, "ERR: before init model_data should be None")

    main.load_model("./models/giberish_model_v1.pkl")

    self.assertNotEqual(main.model_data, None, "ERR: Data should not be None")

  def test_validation(self):
    test_cases = [{"message": "sdfghjklsakgd ahgdjahsgd aidgashdg", "valid": False}, 
                  {"message": "asjdhg %^^SHAD asdp", "valid": False}, 
                  {"message": "asjhdg gshd agdshsuid ahdhd ahdsgsagd ahsdgdagdsa jahgda aoddgdw dhsgda ajdhgd", "valid": False},
                  {"message": "kaughsdf akhdgauygdfk a jdas akgkdaskhgadsg adshjsadkjghasdkjgh aigkyuagd ajkhgas", "valid": False},
                  {"message": "kawgfydk da8668dagidah237937979ah akdakhjgak a9daiy3ygjhads p3qyi", "valid": False},
                  {"message": "adjkljhkad [da[i da9938l ua gly7h3 a,h dahj,ad,hj", "valid": False},
                  {"message": "                    ", "valid": False},
                  {"message": "", "valid": False},
                  {"message": "I was out the other day with dad walking in the park, It was kinda cool honeslty", "valid": True},
                  {"message": "Some wordsoup musolini eats pizza pirate crystal python numbers real fake", "valid": True},
                  {"message": "Now dis iz stupd englich but I hops it wurks, plz bb boi", "valid": True},
                  {"message": "L0l 1 am c00l s0 I typ3 w1th numb3rs", "valid": True},
                  {"message": "please contact me on perros@hotmail.com", "valid": True},
                  {"message": "e e e e e e e e e e e e e", "valid": True}
                ]
    
    main.load_model("./models/giberish_model_v1.pkl")

    for test in test_cases:
      self.assertEqual(main.validate_message(test["message"]), test["valid"], f'ERR: {test["message"]} should be {test["valid"]}')


if __name__ == "__main__":
  unittest.main()