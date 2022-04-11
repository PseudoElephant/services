import Vue from "vue";
import Vuex from "vuex";
import sagas from "@/sagas/index";
import secrets from "./modules/secrets/index";
import config from "./modules/config/index";
import { runSaga, RunSagaOptions, stdChannel } from "redux-saga";

Vue.use(Vuex);

const store = new Vuex.Store({
  modules: {
    secrets,
    config,
  },
});

// dispatch
export const dispatch = (output: DispatchOutput<any>): void =>
  store.commit(output.type, output.payload);
// getState
export const getState = (): State => store.state;

sagas.forEach((saga) => {
  const channel = stdChannel();

  store.subscribeAction(channel.put);

  const options: RunSagaOptions<unknown, State> = {
    channel,

    dispatch,

    getState,
  };

  runSaga(options, saga);
});

export default store;
