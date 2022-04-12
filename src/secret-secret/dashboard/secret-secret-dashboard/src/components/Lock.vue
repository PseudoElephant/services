<template>
    <LockIcon :class="['lock', { 'lock--open': opened }]" />
</template>

<script lang="ts">
import Vue from "vue";
import LockIcon from "@/assets/icons/OpenLock.svg"

export default Vue.extend({
    name: "Lock",
    components: {
        LockIcon
    },
    props: {
       opened : {
           type: Boolean,

       }
    }  
})
</script>

<style lang="scss">
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

</style>