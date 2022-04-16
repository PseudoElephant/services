import * as mutations from "./mutations";
import * as actions from "./actions";
import state from "./state";
import { Module } from "vuex";

const secrets: Module<ConfigState, State> = {
  namespaced: true,
  mutations,
  actions,
  state,
};

export default secrets;
