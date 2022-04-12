<template>
      <div class="controls">
      <div class="controls_social">
        <v-btn icon> <v-icon> mdi-facebook </v-icon></v-btn>
        <v-btn icon> <v-icon> mdi-whatsapp </v-icon></v-btn>
        <v-btn icon> <v-icon> mdi-instagram </v-icon></v-btn>
        <v-btn icon @click="twitterHandle"> <v-icon> mdi-twitter </v-icon></v-btn>
      </div>
      <div class="controls__likes">
        <v-btn icon @click="liked">
          <v-icon> mdi-cards-heart-outline </v-icon></v-btn
        >
        {{ secret.likes }}
      </div>
      
    </div>
</template>



<script lang="ts">
import Vue from "vue";
import { mapState } from "vuex";

export default Vue.extend({
    name: "Controls",
    computed: {
          ...mapState("secrets", ["secret"]),
    },
    methods: {
      liked() {
        console.log("liked")
      },
      twitterHandle() {
        window.open("https://twitter.com/intent/tweet?hashtags=SecretSecret&original_referer=http%3A%2F%2Flocalhost%3A8080%2F&ref_src=twsrc%5Etfw%7Ctwcamp%5Ebuttonembed%7Ctwterm%5Ehashtag%7Ctwgr%5ESecretSecret&text=Look%20at%20this%20amazing%20website...&url=localhost%3A8080", "__blank")
      }
    }
})

</script>

<style lang="scss">
.controls {
  display: flex;
  justify-content: space-between;

  &__likes {
    display: flex;
    flex-direction: column;
    text-align: center;
    color: var(--light-color-1);
  }
}
</style>

