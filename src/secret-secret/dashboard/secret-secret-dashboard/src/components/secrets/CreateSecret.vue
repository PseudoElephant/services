<template>
    <div class="create-secret">
    <v-form v-model="valid">
      <v-textarea
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
        v-model="value"
      />
    </v-form>
    <v-btn class="submit-btn" :disabled="!valid" @click="createSecretRequest">Submit</v-btn>
 </div>
</template>

<script lang="ts">
import Vue from "vue";
import { mapActions } from "vuex";

export default Vue.extend({
name: "CreateSecret",
props: {
  maxlength: {
    type: String,
    default: "300"
  }
},
data() {
  return {
    valid : false,
    value : ""
  }
},
methods: {
   ...mapActions("secrets", ["setSecret", "createSecret"]),
      validateText(text: string): boolean | string {
      if (text == "") {
        return "Secret can't be empty.";
      }

      return true;
    },
    createSecretRequest() : void {
      this.createSecret({ message: this.value});
      console.log("creating")
      this.$emit("sent");
    }
    
},
mounted() {
  this.setSecret({})
}
})
</script>

<style lang="scss">
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

.v-messages__message {
  color: rgb(241, 85, 85);
}
</style>

<style lang="scss" scoped>

.create-secret {
  display: grid;

}

.submit-btn {
  width: 60%;
  margin: auto;
  transition: all 1s ease-in-out;
}

.v-btn__content {
  font-size: 1.3rem;
}
</style>