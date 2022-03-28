<template>
  <div id="app">
    <Home />
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Home from "@/components/Home.vue";

export default Vue.extend({
  name: "App",
  components: {
    Home,
  },
});
</script>

<style>
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
  font-family: "Lucida Sans", "Lucida Sans Regular", "Lucida Grande",
    "Lucida Sans Unicode", Geneva, Verdana, sans-serif;
}
</style>
