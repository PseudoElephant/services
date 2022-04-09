from __future__ import print_function
import pickle
import os 

model_data = None
WORD_VALIDATION_TRESHOLD = float(os.environ.get("WORD_VALIDATION_TRESHOLD", 0.6))
MESSAGE_VALIDATION_TRESHOLD = float(os.environ.get("MESSAGE_VALIDATION_TRESHOLD", 0.6))

def load_model(filename : str):
  global model_data

  if model_data:
    return
      
  try:
    with open(filename, 'rb') as f:
      model_data = pickle.load(f)
  except OSError as e:
    # TODO: Should Log the error and panic!
    pass


def lambda_handler(event, context):
    filename = "./models/giberish_model_v1.pkl"
    load_model(filename)


def validate_message(msg: str) -> bool:
  if not model_data:
    # TODO: Should log err, as model is not loaded properly
    return False

  cv, clf = model_data

  clean = map(lambda x: x.lower(), msg.split())
  
  try:
    test_cv = cv.transform(clean)
    probs = clf.predict_proba(test_cv)
  except ValueError as e:
    # TODO: Log error 
    return False

  res = list(map(lambda x: 0 if x[0] > WORD_VALIDATION_TRESHOLD else 1, probs))
 
  if len(res) == 0:
    return False

  return sum(res) / len(res) > MESSAGE_VALIDATION_TRESHOLD

  
