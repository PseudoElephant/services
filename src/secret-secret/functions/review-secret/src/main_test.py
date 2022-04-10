import unittest
from moto import mock_dynamodb
import main


class TestModel(unittest.TestCase):
    def test_load_model(self):
        main.model_data = main.load_model("./models/giberish_model_v1.pkl")

        self.assertNotEqual(main.model_data, None, "ERR: Data should not be None")

    def test_validation(self):
        test_cases = [
            {"message": "sdfghjklsakgd ahgdjahsgd aidgashdg", "valid": False},
            {"message": "asjdhg %^^SHAD asdp", "valid": False},
            {
                "message": "asjhdg gshd agdshsuid ahdhd ahdsgsagd ahsdgdagdsa jahgda aoddgdw dhsgda ajdhgd",
                "valid": False,
            },
            {
                "message": "kaughsdf akhdgauygdfk a jdas akgkdaskhgadsg adshjsadkjghasdkjgh aigkyuagd ajkhgas",
                "valid": False,
            },
            {
                "message": "kawgfydk da8668dagidah237937979ah akdakhjgak a9daiy3ygjhads p3qyi",
                "valid": False,
            },
            {
                "message": "adjkljhkad [da[i da9938l ua gly7h3 a,h dahj,ad,hj",
                "valid": False,
            },
            {"message": "                    ", "valid": False},
            {"message": "", "valid": False},
            {
                "message": "I was out the other day with dad walking in the park, It was kinda cool honeslty",
                "valid": True,
            },
            {
                "message": "Some wordsoup musolini eats pizza pirate crystal python numbers real fake",
                "valid": True,
            },
            {
                "message": "Now dis iz stupd englich but I hops it wurks, plz bb boi",
                "valid": True,
            },
            {"message": "L0l 1 am c00l s0 I typ3 w1th numb3rs", "valid": True},
            {"message": "please contact me on perros@hotmail.com", "valid": True},
            {"message": "e e e e e e e e e e e e e", "valid": True},
        ]

        main.model_data = main.load_model("./models/giberish_model_v1.pkl")

        for test in test_cases:
            self.assertEqual(
                main.validate_message(test["message"]),
                test["valid"],
                f'ERR: {test["message"]} should be {test["valid"]}',
            )

    @mock_dynamodb
    def test_batch_write(self):
        secrets = [
            {
                "message": "Some Message",
                "secret_id": "some-id",
                "user_id": "some-id",
                "likes": "0",
                "dislikes": "0",
            }
        ]

        try:
            main.batch_create_secret(secrets)
        except Exception as e:
            self.fail(f"batch_create_secret() raised Exception unexpectedly!\n {e}")

    @mock_dynamodb
    def test_handler(self):
        event = {
            "Records": [
                {
                    "messageId": "19dd0b57-b21e-4ac1-bd88-01bbb068cb78",
                    "receiptHandle": "MessageReceiptHandle",
                    "body": '{  "message": "Some Message",  "secret_id": "some-id", "user_id": "some-id",   "user_id": "some-id", "likes": "0",  "dislikes": "0"  }',
                    "attributes": {
                        "ApproximateReceiveCount": "1",
                        "SentTimestamp": "1523232000000",
                        "SenderId": "123456789012",
                        "ApproximateFirstReceiveTimestamp": "1523232000001",
                    },
                    "messageAttributes": {},
                    "md5OfBody": "7b270e59b47ff90a553787216d55d91d",
                    "eventSource": "aws:sqs",
                    "eventSourceARN": "arn:aws:sqs:us-west-2:123456789012:MyQueue",
                    "awsRegion": "us-west-2",
                }
            ]
        }

        try:
            main.lambda_handler(event, None)
        except Exception as e:
            self.fail(f"batch_create_secret() raised Exception unexpectedly!\n {e}")


if __name__ == "__main__":
    unittest.main()
