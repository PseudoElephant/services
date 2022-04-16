import Vue from "vue";
import App from "@/App.vue";
import vuetify from "./plugins/vuetify";
import store from "./stores/index";
import router from "./router/routes";

Vue.config.productionTip = false;
new Vue({
  store,
  vuetify,
  router,
  render: (h) => h(App),
}).$mount("#app");
