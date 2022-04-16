<template>
  <div class="container">
    <div :class="['home', { 'home--post': !creatingSecret }]">
      <div class="title">
        <span>Secret</span><span><Lock :opened="!creatingSecret" /></span
        ><span>Secret</span>
      </div>
      <div class="home__content">
        <!-- Secret Secret Title -->

        <div class="secret">
          <!-- Secret Secret Textarea -->
          <transition name="fade">
            <CreateSecret
              v-if="creatingSecret"
              @sent="() => (creatingSecret = false)"
            />
            <SecretCard v-else @back="() => (creatingSecret = true)" />
          </transition>
        </div>
      </div>

      <!-- Google Ad -->
      <!-- <GoogleAdSense adSlot="2612580659" addFormat="rspv"/> -->
    </div>

    <footer>
      <h2>Secret Secret</h2>
      <div class="link">
        <router-link to="/about">About</router-link>
        <router-link to="/cookie-policy">Cookie Notice</router-link>
      </div>
    </footer>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Lock from "@/components/secrets/Lock.vue";
import SecretCard from "@/components/SecretCard.vue";
import CreateSecret from "@/components/CreateSecret.vue";

// import GoogleAdSense from "@/components/ads/GoogleAdSense.vue"

export default Vue.extend({
  name: "Home",
  components: {
    Lock,
    SecretCard,
    CreateSecret,

    // GoogleAdSense
  },
  data() {
    return {
      creatingSecret: true,
      valid: false,
    };
  },
});
</script>

<style lang="scss">
footer {
  padding: 1rem 2rem;
  flex: 0 1 2rem;
  color: white;
  background: rgb(0, 0, 0);
  display: flex;
  justify-content: space-between;
  z-index: 1;

  h2 {
    font-size: 2rem;
    align-self: center;
  }
}

.link {
  display: flex;
  align-items: center;
}

.link > a {
  margin: 0 1rem;
  text-decoration: none;
  color: white;
  font-size: 1.5rem;
  transition: all 0.3s;

  &:hover {
    color: rgb(90, 90, 226);
  }
}

.fade-enter-active {
  transition: opacity 1s ease-in-out;
}

.fade-enter-to {
  opacity: 1;
}

.fade-enter,
.fade-leave {
  opacity: 0;
}

.v-btn > .v-btn__content .v-icon {
  color: var(--light-color-1) !important;
}
</style>

<style lang="scss" scoped>
.secret {
  > * {
    margin: auto;
    width: 60%;
  }
}

.container {
  height: 100%;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.lock {
  width: 3rem;
}
.home {
  width: 100%;
  // min-height: 100vh;
  flex: 1 1 auto;

  padding: 2rem;
  // Center Items
  display: grid;
  // align-content: center;
  // justify-content: center;

  > :not(:first-child) {
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
    position: fixed;
    // pointer-events: none;
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

.title {
  // Apply to all children
  margin-top: 3rem;
  transition: all 10s ease-in-out;

  > * {
    font-size: 4rem;
    font-weight: 600;
    color: var(--light-color-1);
    align-self: center;
    justify-content: center;
  }

  > :not(:first-child, :last-child) {
    margin: 0 1rem;
  }

  text-align: center;
}

@media (max-width: 768px) {
  .secret {
    > * {
      width: 100%;
    }
  }
  .lock {
    width: 2rem;
  }

  .title {
    > * {
      font-size: 2rem;
    }
  }

  .link > a {
    font-size: 1rem;
  }

  footer {
    h2 {
      font-size: 1.5rem;
    }
  }
}
</style>
