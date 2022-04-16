<template>
  <div class="create-secret">
    <secret-text-area class="sarea" v-model="value" />
    <secret-button class="secret-btn" @click="createSecretRequest"
      >Share Secret</secret-button
    >
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { mapActions } from "vuex";
import SecretTextArea from "./secrets/SecretTextArea.vue";
import SecretButton from "./secrets/SecretButton.vue";

export default Vue.extend({
  components: { SecretTextArea, SecretButton },
  name: "CreateSecret",
  props: {
    maxlength: {
      type: String,
      default: "300",
    },
  },
  data() {
    return {
      valid: false,
      value: "",
    };
  },
  methods: {
    ...mapActions("secrets", ["setSecret", "createSecret"]),
    createSecretRequest(): void {
      this.createSecret({ message: this.value });
      this.$emit("sent");
    },
  },
  mounted() {
    this.setSecret({});
  },
});
</script>

<style lang="scss" scoped>
.create-secret {
  display: grid;

  > * {
    margin: 1rem;
  }
}

.secret-btn {
  width: fit-content;
  place-self: end;
  font-size: 1.4rem;
}

@media (max-width: 768px) {
  .secret-btn {
    font-size: 1.2rem;
  }
}
</style>
