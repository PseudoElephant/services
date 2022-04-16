<template>
  <div class="s-tarea" @focus="onfocus" tabindex="0">
    <textarea
      ref="textarea"
      :maxlength="maxlength"
      :autofocus="autofocus"
      :placeholder="placeholder"
      v-model="modelValue"
      rows="2"
    >
    </textarea>
    <div class="s-tarea__append">{{ counter }}</div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  name: "SecretTextArea",
  props: {
    maxlength: {
      type: Number,
      default: 300,
    },
    autofocus: {
      type: Boolean,
      default: true,
    },
    placeholder: {
      type: String,
      default: "Enter a secret to see a strangers secret...",
    },
    value: {
      type: String,
      default: "",
    },
  },
  computed: {
    counter(): string {
      return `${this.value.length}/${this.maxlength}`;
    },
  },
  data() {
    return {
      modelValue: this.value,
      focused: true,
    };
  },
  methods: {
    onfocus() {
      (this.$refs.textarea as HTMLTextAreaElement).focus();
    },
    oninput(e: InputEvent) {
      e.preventDefault();
    },
  },
  watch: {
    // when value changes from inside notify
    async modelValue(value: string) {
      this.$emit("input", value);

      // Adjust height
      const textarea = this.$refs.textarea as HTMLTextAreaElement;

      const scrollLeft =
        window.pageXOffset ||
        (document.documentElement || document.body.parentNode || document.body)
          .scrollLeft;

      const scrollTop =
        window.pageYOffset ||
        (document.documentElement || document.body.parentNode || document.body)
          .scrollTop;

      textarea.style.height = "auto";
      textarea.style.height = textarea.scrollHeight + "px";

      const scrollLeftOver =
        document.body.clientHeight - scrollTop - outerHeight;

      console.log(outerHeight, scrollTop, document.body.clientHeight);

      let modif = 0;

      // Threshold to go to bottom
      if (scrollLeftOver < 50) {
        modif = scrollLeftOver;
      }

      window.scrollTo(scrollLeft, scrollTop + modif);
    },
    // Change when prop value changes from outiside
    value(value: string) {
      this.modelValue = value;
    },
  },
  mounted() {
    const textarea = this.$refs.textarea as HTMLTextAreaElement;

    textarea.style.height = "auto";
    textarea.style.height = textarea.scrollHeight + "px";
  },
});
</script>

<style lang="scss" scoped>
.s-tarea {
  border: 1px solid rgba(255, 255, 255, 0.5);
  border-radius: 5px;
  transition: all 0.5s ease-in-out;
  color: var(--light-color-1);

  &__append {
    padding: 1rem;
    text-align: end;
    opacity: 0.4;
  }

  &:focus-within {
    border: 1px solid rgb(255, 255, 255);
  }

  &:hover {
    box-shadow: 0px 0px 15px rgba(0, 0, 0, 0.459);
  }
}

textarea {
  color: var(--light-color-1);

  padding: 1rem;
  padding-bottom: 0;
  font-size: 1.6rem;
  line-height: 1.3;
  resize: none;
  height: auto;
  width: 100%;
  overflow: hidden;
  outline: none;
  text-align: justify;
  text-justify: inter-word;

  transition: all 1s ease-in-out;

  &::placeholder {
    color: rgba(255, 255, 255, 0.4);
    font-weight: 400;
  }
}

@media (max-width: 768px) {
  textarea {
    font-size: 1rem;
  }
}
</style>
