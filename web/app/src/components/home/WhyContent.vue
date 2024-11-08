<template>
  <div class="why-content">
    <img :src="img" class="why-icon" />
    <div class="title">
      {{ title }}
    </div>
    <div class="text">
      {{ text }}
    </div>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  img: string
  title: string
  text: string
}>()
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.why-content {
  transition: transform 0.25s linear;
  &:hover {
    transform: scale(1.02);
  }
}
.why-icon {
  width: 54px;
  height: 54px;
}
.title {
  @mixin bold 20px;
  color: $purple;
  margin-top: 24px;
}
.text {
  @mixin text 15px;
  color: black;
  margin-top: 6px;
}
@media (max-width: 1000px) {
  .title {
    margin-top: 16px;
    font-size: 20px;
  }
  .text {
    font-size: 14px;
  }
}
</style>
