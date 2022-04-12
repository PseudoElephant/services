<template>
  <div :class="classNames">
    <ins
      class="adsbygoogle"
      width="100wv"
      height="300"
      :style="{display: 'block'}"
      :data-ad-client="googleAdId"
      :data-ad-slot="adSlot"
      :data-ad-format="adFormat"
      data-full-width-responsive="true"
    ></ins>
  </div>
</template>

<script lang="ts">
declare var window: any

import Vue from "vue"
export default Vue.extend({
  name: "GoogleAd",
  props: {
    classNames: String,
    adSlot: String,
    adFormat: String
  },
  data() {
    return {
      googleInit: undefined as number | undefined,
      googleAdId: "ca-pub-2218205034208454"
    };
  },
  mounted() {
  	let timeout = 100;
    this.googleInit = setTimeout(() => {
      if (typeof window !== "undefined")
      {
        (window.adsbygoogle = window.adsbygoogle || []).push({});
        return
      }
    }, timeout);
  },
  destroyed() {
    if (this.googleInit) clearTimeout(this.googleInit);
  }
});
</script>
