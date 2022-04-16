import { createLocalVue, mount, shallowMount } from "@vue/test-utils";
import CreateSecret from "@/components/CreateSecret.vue";
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
  let emitMock: jest.Mock;
  let storeMock: Store<State>;

  beforeEach(() => {
    localVue = createLocalVue();
    localVue.use(Vuex);
    localVue.use(Vuetify);

    setSecretMock = jest.fn();
    createSecretMock = jest.fn();
    fetchSecretMock = jest.fn();
    emitMock = jest.fn();
    storeMock = new Vuex.Store({
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
  });

  it("mount component", async () => {
    const wrapper = shallowMount(CreateSecret, {
      localVue,
      store: storeMock,
      vuetify: new Vuetify(),
      mocks: {
        $emit: emitMock,
      },
    });

    expect(setSecretMock).toBeCalledTimes(1);

    expect(wrapper.findAll(".create-secret").length).toBe(1);
  });

  it("create secret", async () => {
    const wrapper = shallowMount(CreateSecret, {
      localVue,
      store: storeMock,
      vuetify: new Vuetify(),
      mocks: {
        $emit: emitMock,
      },
    });

    (wrapper.vm as any).createSecretRequest();

    await Vue.nextTick();

    expect(createSecretMock).toBeCalled();
    expect(emitMock).toHaveBeenLastCalledWith("sent");
  });

  it("validate text", async () => {
    const wrapper = shallowMount(CreateSecret, {
      localVue,
      store: storeMock,
      vuetify: new Vuetify(),
      mocks: {
        $emit: emitMock,
      },
    });

    let validation = (wrapper.vm as any).validateText("");
    expect(validation).toBe("Secret can't be empty.");

    validation = (wrapper.vm as any).validateText("Some secret message");
    expect(validation).toBe(true);
  });
});
