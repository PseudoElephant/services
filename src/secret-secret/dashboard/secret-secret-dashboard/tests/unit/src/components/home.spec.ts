import { createLocalVue, shallowMount } from "@vue/test-utils";
import Home from "@/views/Home.vue";
import { VueConstructor } from "vue";
import Vuetify from "vuetify";
import Vue from "vue";

Vue.use(Vuetify);

describe("home component", () => {
  let localVue: VueConstructor<Vue>;

  beforeEach(() => {
    localVue = createLocalVue();

    localVue.use(Vuetify);
  });

  it("mount component", () => {
    const wrapper = shallowMount(Home, {
      localVue,
    });

    expect(wrapper.findAll(".home").length).toBe(1);
  });
});
