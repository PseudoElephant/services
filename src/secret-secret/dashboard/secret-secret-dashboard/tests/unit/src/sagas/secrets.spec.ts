import sagaHelper from "redux-saga-testing";
import secretsSaga, { createSecret, fetchRandomSecret } from "@/sagas/secrets";

describe("secrets saga", () => {
  const it = sagaHelper(secretsSaga());

  it("fetchRandomSecret", (takeLatest) => {
    expect(takeLatest.payload.args[0]).toBe("secrets/fetchSecret");
    expect(takeLatest.payload.args[1].name).toBe("fetchRandomSecret");
  });

  it("createSecret", (takeLatest) => {
    expect(takeLatest.payload.args[0]).toBe("secrets/createSecret");
    expect(takeLatest.payload.args[1].name).toBe("createSecret");
  });
});

describe("createSecret Saga", () => {
  const it = sagaHelper(fetchRandomSecret());

  it("setLoading to true", (put) => {
    expect(put.payload.action.type).toBe("config/setLoading");
    expect(put.payload.action.payload).toBe(true);
  });

  it("getSecret", (call) => {
    expect(call.payload.args[0].method).toBe("get");
    return {
      data: {},
    };
  });

  it("createSecret", (put) => {
    expect(put.payload.action.type).toBe("secrets/setSecret");
    expect(put.payload.action.payload).toStrictEqual({} as Secret);
  });

  it("setLoading to true", (put) => {
    expect(put.payload.action.type).toBe("config/setLoading");
    expect(put.payload.action.payload).toBe(false);
  });
});

describe("createSecret Saga", () => {
  const it = sagaHelper(
    createSecret({
      payload: {} as Secret,
      type: "",
    })
  );

  it("createSecret", (call) => {
    expect(call.payload.args[0].method).toBe("post");
    return {
      data: {},
    };
  });
});
