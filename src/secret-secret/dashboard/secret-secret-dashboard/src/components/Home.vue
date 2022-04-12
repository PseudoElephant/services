<template>
  <div :class="['home', { 'home--post': !creatingSecret }]">
    <!-- Secret Secret Title -->
    <div class="title">
      <span>Secret</span><span><Lock :opened="!creatingSecret" /></span
      ><span>Secret</span>
    </div>
 
  <div>
    <!-- Secret Secret Textarea -->
     <transition  name="fade">
      <CreateSecret  v-if="creatingSecret"  @sent="() => creatingSecret = false" />
      <SecretCard v-else @back="() => creatingSecret = true"  />
     </transition>
</div>
<!-- Google Ad -->
    <!-- <GoogleAdSense adSlot="2612580659" addFormat="rspv"/> -->
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Lock from "@/components/Lock.vue";
import SecretCard from "@/components/secrets/SecretCard.vue";
import CreateSecret from "@/components/secrets/CreateSecret.vue";
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
      valid: false
    };
  }
});
</script>

<style lang="scss">

.fade-enter-active {
  transition: opacity 0.5s ease-in-out;

}

.fade-enter-to  {
  opacity: 1;

}

.fade-enter, .fade-leave{
  opacity: 0;
}


.v-btn > .v-btn__content .v-icon {
  color: var(--light-color-1) !important;
}
</style>

<style lang="scss" scoped>
.home {
  width: 100%;
  min-height: 100vh;
  height: auto;

  padding: 2rem;
  // Center Items
  display: inline-grid;
  // align-content: center;
  justify-content: center;
  overflow: scroll;

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
