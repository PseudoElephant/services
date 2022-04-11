import * as actions from "./actions";
import * as getters from "./getters";
import * as mutations from "./mutations";
import state from "./state";
import { Module } from "vuex";

const secrets: Module<SecretsState, State> = {
  namespaced: true,
  actions,
  getters,
  mutations,
  state,
};

export default secrets;
