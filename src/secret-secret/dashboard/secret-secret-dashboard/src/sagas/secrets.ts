import { call, put, StrictEffect, takeLatest } from "redux-saga/effects";
import { BASE_API } from "@/config/constants";
import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
// fetchRandomSecret retrieves a random secret from the backend
function* fetchRandomSecret(): Generator<StrictEffect> {
  try {
    // setLoading
    yield put({ type: "config/setLoading", payload: true });
    // make axios request
    const requestConfig: AxiosRequestConfig = {
      method: "get",
      baseURL: BASE_API,
      url: "/secrets",
    };
    // making request
    const response = (yield call(
      axios.request,
      requestConfig
    )) as AxiosResponse;

    // handle invalid status
    const secret = response.data as Secret;
    // store secret in store
    yield put({ type: "secrets/setSecret", payload: secret });
  } catch (e) {
    // notify errors
  } finally {
    //  setLoading false
    yield put({ type: "config/setLoading", payload: false });
  }
}

function* createSecret(
  request: DispatchOutput<Secret>
): Generator<StrictEffect> {
  try {
    // make axios request
    const requestConfig: AxiosRequestConfig = {
      method: "post",
      data: request.payload,
      headers: {
        "content-type": "application/json",
      },
      baseURL: BASE_API,
      url: "/secrets",
    };

    // making request
    const response = (yield call(
      axios.request,
      requestConfig
    )) as AxiosResponse;
    console.log(response);
  } catch (e) {
    // some
  } finally {
    //  setLoading false
  }
}

// secretsSaga logic to  handle secret sagas
export default function* secretsSaga(): Generator<StrictEffect> {
  yield takeLatest("secrets/fetchSecret", fetchRandomSecret);
  yield takeLatest("secrets/createSecret", createSecret);
}
