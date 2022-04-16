<template>
  <div class="show-secret">
    <div class="card">
      <span class="text-normal">
        {{ secret.message }}
      </span>

      <v-progress-linear dark v-if="loading" indeterminate></v-progress-linear>
    </div>
    <secret-button class="s-btn" @click="back"
      >Share Another Secret</secret-button
    >
  </div>
</template>

<script lang="ts">
import Vue from "vue";

import { mapActions, mapState } from "vuex";
import SecretButton from "./secrets/SecretButton.vue";

export default Vue.extend({
  name: "SecretCard",
  components: { SecretButton },
  computed: {
    ...mapState("secrets", ["secret"]),
    ...mapState("config", ["loading"]),
  },
  methods: {
    ...mapActions("secrets", ["fetchSecret"]),
    back() {
      this.$emit("back");
    },
  },
  beforeMount() {
    this.fetchSecret();
  },
});
</script>

<style lang="scss">
.v-progress-linear__indeterminate--active {
  background-color: var(--light-color-1);
}
</style>

<style lang="scss" scoped>
.card {
  background-color: rgba(255, 255, 255, 0.4);
  border: 1px solid white;
  color: white;
  padding: 1rem;

  .text-normal {
    font-size: 1.6rem;
  }
}

.show-secret {
  display: grid;
  > * {
    margin: 1rem;
  }
}

.s-btn {
  place-self: end;
  border-radius: 0px;
  font-size: 1.4rem;
}

@media (max-width: 768px) {
  .s-btn {
    font-size: 1.2rem;
  }

  .card {
    .text-normal {
      font-size: 1rem;
    }
  }
}
</style>
