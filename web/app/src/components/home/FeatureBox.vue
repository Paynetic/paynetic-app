<template>
  <div ref="box" class="feature-box" :class="{ reverse, show }">
    <div class="box-left f-center-col">
      <div class="box-title">
        {{ title }}
      </div>
      <div class="box-text">
        {{ text }}
      </div>
    </div>
    <div class="box-right f-center">
      <img :src="img" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, nextTick, onUnmounted, ref } from 'vue'

defineProps<{
  title: string
  text: string
  img: string
  reverse?: boolean
}>()

let observer: IntersectionObserver | undefined
const show = ref(false)
const box = ref()

const observed = (entries: IntersectionObserverEntry[], _: IntersectionObserver) => {
  const obs = entries[0]
  if (obs?.isIntersecting) {
    show.value = true
  }
}

onMounted(async () => {
  if (!observer) {
    const options = {
      root: null,
      rootMargin: '0px',
      threshold: 0.6,
    }
    observer = new IntersectionObserver(observed, options)
    observer.observe(box.value)
  }
  setTimeout(() => {
    console.log(
      box.value.getBoundingClientRect().top,
      box.value.scrollTop,
      window.scrollY,
    )
    if (box.value.getBoundingClientRect().top < 0) {
      show.value = true
    }
  }, 100)
})

onUnmounted(() => {
  observer?.disconnect()
  observer = undefined
})
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.feature-box {
  display: flex;
  position: relative;
  width: 100%;
  height: 300px;
  margin-top: 16px;
  opacity: 0;
  left: -60px;
  transition: all 0.25s ease;
  &.reverse {
    flex-direction: row-reverse;
    left: 60px;
    .box-left {
      background: linear-gradient(
        245deg,
        rgba(181, 216, 205, 0.25) 0%,
        rgb(181, 216, 205) 100%
      );
    }
  }
  &.show {
    left: 0;
    opacity: 1;
  }
}

.box-left {
  background: linear-gradient(
    115deg,
    rgba(181, 216, 205, 0.25) 0%,
    rgb(181, 216, 205) 100%
  );
  width: 65%;
  text-align: left;
  padding: 0 32px 0 64px;
  align-items: flex-start;
}
.box-title {
  @mixin bold 20px;
  color: $text2;
}
.box-text {
  @mixin text 16px;
  margin-top: 16px;
}

.box-right {
  width: 35%;
  background-color: rgba(130, 207, 199, 0.1);
  img {
    width: 200px;
    height: 200px;
    mix-blend-mode: multiply;
  }
}

@media (max-width: 800px) {
  .box-right {
    img {
      width: 140px;
      height: 140px;
    }
  }
}
@media (max-width: 580px) {
  .feature-box {
    flex-direction: column-reverse;
    height: auto;
    &.reverse {
      flex-direction: column-reverse;
    }
  }
  .box-left {
    width: 100%;
    padding-top: 64px;
    padding-bottom: 64px;
  }
  .box-right {
    width: 100%;
    height: 180px;
  }
}
</style>
