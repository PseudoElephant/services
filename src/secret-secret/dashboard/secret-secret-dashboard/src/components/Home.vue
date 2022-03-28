<template>
  <div :class="['home', { 'home--post': isSendSecretView }]">
    <div class="title">
      <span>Secret</span><span> <v-icon dark x-large> mdi-lock </v-icon></span
      ><span>Secret</span>
    </div>
    <v-textarea
      class="textarea"
      counter="300"
      no-resize
      rows="5"
      dark
      shaped
      filled
      v-model="secretMessage"
    ></v-textarea>
    <div class="home__controls">
      <v-btn icon color="pink" @click="toggleSecretView">
        <v-icon dark> mdi-heart </v-icon></v-btn
      >
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  name: "Home",
  data() {
    return {
      isSendSecretView: false,
      secretMessage: "",
    };
  },
  methods: {
    toggleSecretView() {
      this.isSendSecretView = !this.isSendSecretView;
      console.log(this.secretMessage);
    },
  },
});
</script>

<style>
.v-btn > .v-btn__content .v-icon {
  color: rgb(236, 16, 71) !important;
}

textarea {
  font-size: 2rem !important;
  line-height: 3rem !important;
}

textarea::-webkit-scrollbar {
  background-color: rgba(255, 255, 255, 0);
  width: 12px;
}

/* background of the scrollbar except button or resizer */
textarea::-webkit-scrollbar-track {
  background-color: rgba(255, 255, 255, 0);
}

/* scrollbar itself */
textarea::-webkit-scrollbar-thumb {
  background-color: #babac049;
  border-radius: 16px;
  border: 1px solid #fff;
}

/* set button(top and bottom of the scrollbar) */
body::-webkit-scrollbar-button {
  display: none;
}
</style>

<style lang="scss" scoped>
.home {
  width: 100%;
  height: 100vh;

  padding: 2rem;
  // Center Items
  display: grid;
  align-content: center;
  justify-content: center;

  > :not(:first-child, :last-child) {
    margin-top: 3rem;
  }

  // Background Effect
  background-image: radial-gradient(
    var(--radial-color-1),
    var(--radial-color-2)
  );
  z-index: 1;
  position: relative;

  &::before {
    position: absolute;
    content: "";
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    background-image: radial-gradient(
      var(--radial-color-3),
      var(--radial-color-4)
    );
    z-index: -1;
    opacity: 0;

    transition: opacity 1s linear;
  }

  &--post::before {
    opacity: 1;
  }
}

// SVG animation
// .lock-icon {
//   &:nth-child {
//     animation: growShrink 2s ease-in;
//   }
// }

// @keyframes growShrink {
//   0% {
//     height: 100%;
//   }
//   100% {
//     height: 0;
//   }
// }

.title {
  // Apply to all children
  > * {
    font-size: 4rem;
    font-weight: 600;
    color: var(--light-color-1);
    align-self: center;
    justify-content: center;
  }

  > :not(:first-child, :last-child) {
    margin: 0 2rem;
  }

  margin: auto;
}
</style>
