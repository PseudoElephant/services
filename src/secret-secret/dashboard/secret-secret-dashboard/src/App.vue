<template>
  <div id="app">
    <!-- route outlet -->
    <!-- component matched by the route will render here -->
    <router-view></router-view>
    <cookie-banner class="banner" v-if="cookiePreference == null" />
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import CookieBanner from "@/components/CookieBanner.vue";
import { mapActions, mapState } from "vuex";
import { COOKIE_PREFERENCE } from "./config/constants";

export default Vue.extend({
  name: "App",
  components: {
    CookieBanner,
  },
  computed: {
    ...mapState("config", ["cookiePreference"]),
  },
  methods: {
    ...mapActions("config", ["setCookiePreference"]),
  },
  beforeMount() {
    // Load Cookie Preferences
    const preference = localStorage.getItem(COOKIE_PREFERENCE);

    if (preference) {
      this.setCookiePreference(Boolean(preference));
    }
  },
});
</script>

<style>
@import url("https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,100;0,300;0,400;0,700;0,900;1,100;1,300;1,400;1,700;1,900&display=swap");

:root {
  --light-color-1: white;
  --radial-color-1: rgb(35, 35, 156);
  --radial-color-2: rgb(29, 23, 114);
  --radial-color-3: rgb(112, 35, 156);
  --radial-color-4: rgb(69, 20, 92);
}

*,
body,
span,
html {
  margin: 0px;
  padding: 0px;
  font-size: 10px;
  font-family: "Lato", sans-serif;
}

.banner {
  z-index: 1;
}
</style>
