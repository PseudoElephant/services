from __future__ import print_function
import pickle

model_data = None

def load_model(filename : str):
  try:
    with open(filename, 'rb') as f:
      return pickle.load(f)
  except OSError as e:
    # Should Log the error and panic!
    pass


def lambda_handler(event, context):
    global model_data

    filename = "./models/giberish_model_v1.pkl"
    
    if not model_data:
      model_data = load_model(filename)

    cv, clf = model_data

