import { createLocalVue, shallowMount } from "@vue/test-utils";
import Home from "@/components/Home.vue";
import { VueConstructor } from "vue";
import Vuex, { Store } from "vuex";
import Vuetify from "vuetify";
import Vue from "vue";
import secrets from "@/stores/modules/secrets";
import config from "@/stores/modules/config";

Vue.use(Vuetify);

describe("home component", () => {
  let localVue: VueConstructor<Vue>;
  let setSecretMock: jest.Mock;
  let createSecretMock: jest.Mock;
  let fetchSecretMock: jest.Mock;

  beforeEach(() => {
    localVue = createLocalVue();
    localVue.use(Vuex);
    localVue.use(Vuetify);

    setSecretMock = jest.fn();
    createSecretMock = jest.fn();
    fetchSecretMock = jest.fn();
  });

  it("mount component", () => {
    const store: Store<State> = new Vuex.Store({
      modules: {
        secrets: {
          ...secrets,
          namespaced: true,
          actions: {
            ...secrets.actions,
            setSecret: setSecretMock,
            createSecret: createSecretMock,
            fetchSecret: fetchSecretMock,
          },
        },
        config: {
          ...config,
        },
      },
    });
    const wrapper = shallowMount(Home, {
      localVue,
      store,
    });

    expect(wrapper.findAll(".home").length).toBe(1);
  });
});
