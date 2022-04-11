import * as mutations from "./mutations";
import state from "./state";
import { Module } from "vuex";

const secrets: Module<ConfigState, State> = {
  namespaced: true,
  mutations,
  state,
};

export default secrets;
