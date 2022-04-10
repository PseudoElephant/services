"""
Lambda that determines if a given text is valid.
"""

import json
import pickle
import os
import boto3
import logging

dynamodb = boto3.resource("dynamodb")
logger = logging.getLogger()
logger.setLevel(logging.INFO)

SECRET_TABLE_NAME = "secrets-table"
"""
Table name for the secrets
"""

WORD_VALIDATION_TRESHOLD = float(os.environ.get("WORD_VALIDATION_TRESHOLD", 0.6))
"""
Determines the threshold needed to determine a word is invalid
"""
MESSAGE_VALIDATION_TRESHOLD = float(os.environ.get("MESSAGE_VALIDATION_TRESHOLD", 0.6))
"""
Determines the threshold needed to determine if a message is valid
"""
DEFAULT_MODEL_PATH = os.environ.get(
    "DEFAULT_MODEL_PATH", "/opt/models/giberish_model_v1.pkl"
)
"""
The default path for the giberish detector model
"""


def load_model(filename: str) -> None:
    """
    Tries to load the model into a global variable
    """
    try:
        with open(filename, "rb") as f:
            return pickle.load(f)
    except OSError as e:
        logger.exception(f"ERR: {e}")
        return None


model_data = load_model(DEFAULT_MODEL_PATH)


def validate_message(msg: str) -> bool:
    if not model_data:
        logger.error(f"ERR: Model is not loaded!")
        return False

    cv, clf = model_data

    clean = map(lambda x: x.lower(), msg.split())

    try:
        test_cv = cv.transform(clean)
        probs = clf.predict_proba(test_cv)
    except ValueError as e:
        logger.error(f"ERR: {e}")
        return False

    res = list(map(lambda x: 0 if x[0] > WORD_VALIDATION_TRESHOLD else 1, probs))

    if len(res) == 0:
        return False

    return sum(res) / len(res) > MESSAGE_VALIDATION_TRESHOLD


def batch_create_secret(secrets):
    table = dynamodb.Table(SECRET_TABLE_NAME)

    with table.batch_writer() as batch:
        for secret in secrets:
            batch.put_item(Item=secret)
    logger.info("Batch writer succesfully uploaded secrets to dynamodb.")


def lambda_handler(event, context):
    # Try to load the model,if already loaded it will not do anything.
    global model_data

    if not model_data:
        model_data = load_model(DEFAULT_MODEL_PATH)

    secrets = []
    for record in event["Records"]:
        payload = record["body"]
        secrets.append(json.loads(str(payload)))

    # marshal to secret

    # evaluate secrets
    secrets = filter(lambda x: validate_message(x.get("message", "")), secrets)

    # update dynamo
    try:
        batch_create_secret(secrets)
    except Exception as e:
        logger.exception(e)
