<template>
  <div class="faq-item" :class="{ open }">
    <div class="faq-question" @click="open = !open">
      <div class="question-text">
        {{ question }}
      </div>
      <div class="toggle f-center">
        <div class="horizontal"></div>
        <div class="vertical"></div>
      </div>
    </div>
    <div class="faq-answer" v-html="answer"></div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

defineProps<{
  question: string
  answer: string
}>()

const open = ref(false)
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.faq-item {
  position: relative;
  max-width: 912px;
  width: 100%;
  margin: 16px auto 0;
}
.faq-question {
  @mixin bold 17px;
  display: flex;
  cursor: pointer;
  padding: 16px 25px;
  background-color: rgb(181, 216, 205);
  color: black;
  transition: all 0.25s ease;
  &:hover {
    background-color: rgb(130, 206, 199);
  }
}
.toggle {
  width: 20px;
  height: 20px;
  transition: all 0.25s ease;
  transform: rotate(0);
  position: relative;
  margin-left: auto;
}
.horizontal {
  background-color: $text2;
  width: 100%;
  height: 2px;
  border-radius: 2px;
  opacity: 1;
  transition: all 0.25s ease;
}
.vertical {
  position: absolute;
  background-color: $text2;
  width: 2px;
  height: 100%;
  border-radius: 2px;
  transition: all 0.25s ease;
}
.faq-answer {
  @mixin text 16px;
  display: none;
  padding: 15px 25px;
  background-color: rgba(182, 217, 206, 0.15);
  margin-top: 12px;
  text-align: left;
  line-height: 1.5em;
  :deep(span) {
    font-weight: 700;
  }
}
.faq-item.open {
  .toggle {
    transform: rotate(90deg);
  }
  .faq-question {
    font-weight: 700;
  }
  .faq-answer {
    display: block;
  }
  .horizontal {
    opacity: 0;
  }
}
</style>
