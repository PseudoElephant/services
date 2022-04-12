<template>
  <v-card
    dark
    class="secret-card"
  >
    <v-card-title>
      <span class="title">From Anonymous: </span>
    </v-card-title>

    <v-card-text class="text-normal">
     {{ secret.message }}
     </v-card-text>

    <v-card-actions>
      <Controls class="controls"/>

    </v-card-actions>

    <v-btn icon @click="back" class="back-btn"> <v-icon> mdi-arrow-left </v-icon> </v-btn>
    <v-progress-linear dark v-if="loading" indeterminate></v-progress-linear>
  </v-card>
</template>

<script lang="ts">
import Vue from "vue";
import Controls from "@/components/Controls.vue"
import { mapActions, mapState } from "vuex";

export default Vue.extend({
    name: "SecretCard",
    components: {
      Controls
    },
      computed: {
          ...mapState("secrets", ["secret"]),
          ...mapState("config", [ "loading" ])
    },
    methods: {
      ...mapActions("secrets", ["fetchSecret"]),
      back() {
          this.$emit("back");
      }
    },
    beforeMount() {
      this.fetchSecret()
    }
})
</script>

<style lang="scss">
.v-progress-linear__indeterminate--active {
  background-color: var(--light-color-1);
}
</style>

<style lang="scss" scoped>

.back-btn {
  position: absolute;
  left: 0%;
  top: 0%;
}

.secret-card {
  height: fit-content;
   padding: 2rem;
   max-width: 700px !important;
}

.title {
  font-size: 2rem;
}

.text-normal {
  font-size: 2.4rem !important;
  line-height: 3.2rem !important;
  text-indent: 2rem !important;
 
}

@media (max-width: 768px) {
  .text-normal {
    font-size: 1.5rem !important;
  }

}


.controls {
  width: 100%;
}
</style>