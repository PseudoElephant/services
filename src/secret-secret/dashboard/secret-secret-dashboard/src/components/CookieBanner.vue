<template>
  <div class="cookie-banner">
    <span
      >We use cookies to enhance your browsing experience, serve personalized
      ads or content, and analyze our traffic. By clicking <b>"Accept"</b>, you
      consent to our use of cookies.
      <router-link to="/cookie-policy"> Read More</router-link>
    </span>
    <div class="buttons">
      <secret-button @click="() => setCookie(true)">Accept</secret-button>
      <secret-button @click="() => setCookie(false)">Reject</secret-button>
    </div>
  </div>
</template>

<script lang="ts">
import SecretButton from "./secrets/SecretButton.vue";
import { COOKIE_PREFERENCE } from "@/config/constants";
import { mapActions } from "vuex";

export default {
  components: { SecretButton },
  methods: {
    ...mapActions("config", ["setCookiePreference"]),
    setCookie(accept: boolean): void {
      // Sets Cookie Preference
      localStorage.setItem(COOKIE_PREFERENCE, String(accept));
      this.setCookiePreference(accept);
    },
  },
};
</script>

<style lang="scss" scoped>
.cookie-banner {
  position: fixed;
  bottom: 0;
  left: 0;

  width: 100%;
  background-color: #161010;
  color: white;
  text-align: center;

  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
}

.buttons {
  > * {
    margin: 0 1rem;
  }
}
</style>
