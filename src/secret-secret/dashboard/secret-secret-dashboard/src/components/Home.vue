<template>
  <div :class="['home', { 'home--post': isSendSecretView }]">
    <div class="title">
      <span>Secret</span
      ><span
        ><Lock :class="['lock', { 'lock--open': isSendSecretView }]" /></span
      ><span>Secret</span>
    </div>
    <!-- Try to use forms in vue, see how it works -->
    <v-form v-model="valid">
      <v-textarea
        v-if="!isSendSecretView"
        class="textarea"
        placeholder="A juicy secret goes here..."
        :rules="[validateText]"
        :counter="maxlength"
        :maxlength="maxlength"
        no-resize
        rows="10"
        dark
        shaped
        filled
        v-model="secretMessage"
      />
      <!-- TODO: Review if necesary to have to textareas -->
      <v-textarea
        v-else
        class="textarea"
        no-resize
        rows="10"
        dark
        shaped
        filled
        readonly
        v-model="secretMessage"
      />
    </v-form>
    <v-progress-circular
      indeterminate
      color="primary"
      v-if="loading"
    ></v-progress-circular>
    <div class="controls">
      <div class="controls_social">
        <v-btn icon> <v-icon> mdi-facebook </v-icon></v-btn>
        <v-btn icon> <v-icon> mdi-whatsapp </v-icon></v-btn>
        <v-btn icon> <v-icon> mdi-instagram </v-icon></v-btn>
        <v-btn icon> <v-icon> mdi-atom-variant </v-icon></v-btn>
      </div>
      <div class="controls__likes">
        <v-btn icon @click="toggleSecretView" :disabled="!valid">
          <v-icon> mdi-cards-heart-outline </v-icon></v-btn
        >
        {{ secret.likes }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Lock from "@/assets/icons/OpenLock.svg";
import { mapActions, mapState } from "vuex";

export default Vue.extend({
  name: "Home",
  components: {
    Lock,
  },
  data() {
    return {
      maxlength: 300,
      isSendSecretView: false,
      secretMessage: "",
      valid: false,
    };
  },
  computed: {
    ...mapState("secrets", ["secret"]),
    ...mapState("config", ["loading"]),
  },
  methods: {
    ...mapActions("secrets", ["fetchSecret", "setSecret", "createSecret"]),
    validateText(text: string): boolean | string {
      if (text == "") {
        return "Secret can't be empty.";
      }

      return true;
    },
    toggleSecretView() {
      // Clear State
      this.setSecret({});

      if (!this.isSendSecretView) {
        // Create secret
        this.createSecret({ message: this.secretMessage }); // don't wait
        // load random secret
        this.fetchSecret();
      }

      this.isSendSecretView = !this.isSendSecretView;
    },
  },
  watch: {
    secret(value) {
      this.secretMessage = value.message;
    },
  },
});
</script>

<style lang="scss">
div[role="progressbar"] {
  margin: 0 !important;
}

.v-progress-circular__overlay {
  stroke: rgb(216, 175, 238) !important;
}

.v-messages__message {
  color: rgb(241, 85, 85);
}

.v-btn > .v-btn__content .v-icon {
  color: var(--light-color-1) !important;
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

.lock {
  width: 7%;

  > * {
    fill: var(--light-color-1);
    transition: transform 0.3s ease-in-out;
  }

  > .head {
    transform: translateY(11.09%);
  }

  > .sbody {
    transform: translateY(-5.47%);
  }

  &--open {
    > .head {
      transform: translateY(0%);
    }

    > .sbody {
      transform: translateY(0%);
    }
  }
}

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

  text-align: center;
}
</style>
