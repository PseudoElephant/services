<template>
  <button @click="onclick"><slot /></button>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  name: "SecretButton",
  methods: {
    onclick() {
      console.log("CLICK");
      this.$emit("click");
    },
  },
});
</script>

<style lang="scss" scoped>
button {
  border-radius: 5px;
  border: 1px solid white;
  color: white;
  padding: 0.5rem;
  background-color: rgba(255, 255, 255, 0.2);
  transition: all 0.5s ease-in-out;
  word-spacing: 0.3rem;

  &:hover {
    background-color: rgba(255, 255, 255, 0.4);
  }
}
</style>
