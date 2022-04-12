<template>
  <v-card
    dark
    class="secret-card"
  >
    <v-card-title>
      <span class="title">From Anonymous: </span>
    </v-card-title>

    <v-card-text class="text-normal">
      Lorem ipsum dolor sit amet, consectetur adipisicing elit. Enim voluptate inventore voluptates nostrum illum voluptatibus nulla nam, neque excepturi. Exercitationem nihil, aut molestias, expedita possimus ut porro quae beatae officiis suscipit vel amet reprehenderit cum ullam. Delectus, eius mollitia. Excepturi recusandae dicta vero cumque, laudantium perspiciatis fugit id tempora deserunt inventore mollitia ea aut natus eveniet eligendi quasi? Voluptatem atque, explicabo a earum, minima voluptatibus quaerat dolores expedita doloribus ipsum quam enim neque saepe illum quasi beatae. Magni voluptas voluptatem deleniti iusto velit quae saepe, excepturi rem magnam perferendis voluptate nisi provident, ut ipsum pariatur dolorem tenetur mollitia consectetur esse! Laboriosam maiores totam voluptatibus atque nesciunt dignissimos ad voluptatum consequatur neque dolor aliquid voluptatem laudantium numquam est amet eaque, repudiandae voluptate, quo reiciendis commodi, perferendis veniam unde velit dicta? Laudantium, mollitia, minima necessitatibus aperiam architecto sapiente vitae earum debitis, corrupti doloribus saepe enim quibusdam facilis in quasi veritatis consectetur? Officia, aperiam! Quo, facilis a unde exercitationem dignissimos beatae mollitia sit fugiat quibusdam eos, accusamus eveniet id illum! Nam modi exercitationem atque sunt in ut quibusdam assumenda, deleniti error esse hic earum provident quidem velit molestiae! Quo modi eveniet nesciunt aspernatur sint ullam vel architecto recusandae totam, incidunt, provident at voluptatem illo quas. Tenetur quibusdam beatae qui ex aspernatur exercitationem dignissimos quam ratione voluptates praesentium voluptatibus adipisci perferendis non saepe minus iure doloribus laudantium dolores cumque, nobis eius iste officiis. A, numquam perferendis. At accusantium velit quisquam quis ea cupiditate, rem porro eos eveniet sunt eum enim necessitatibus a quibusdam deserunt. Minima voluptate, illo unde fugiat fuga fugit necessitatibus iusto voluptatum dolor ex, dignissimos cupiditate ad numquam inventore? Deleniti labore suscipit dolor modi iusto optio a quisquam iure molestias, laudantium blanditiis alias sint explicabo sit delectus distinctio animi nobis? Atque voluptatum dolorem sapiente inventore aliquam est nobis asperiores corrupti cupiditate excepturi.
     </v-card-text>

    <v-card-actions>
      <Controls class="controls"/>

    </v-card-actions>

  </v-card>
</template>

<script lang="ts">
import Vue from "vue";
import Controls from "@/components/Controls.vue"

export default Vue.extend({
    name: "SecretCard",
    components: {
      Controls
    }
})
</script>



<style lang="scss">

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