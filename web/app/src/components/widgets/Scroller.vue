<template>
  <div class="img-scroll" :style="{ transform: scrollX }">
    <img v-for="n in scrollCount" :src="img" :style="{ width: `${width}px` }" />
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { useResize, useScroll } from '@app/util-app'

const { width, reverse = false } = defineProps<{
  img: string
  width: number
  reverse?: boolean
}>()

const { innerWidth } = useResize()
const { yPos } = useScroll()

const scrollCount = computed(() => {
  return Math.round(innerWidth.value / (width + 20)) + 1
})

const scrollX = computed(() => {
  let val = -Math.min(400, yPos.value / 9)
  if (reverse) {
    val *= -1
  }
  return `translateX(${val}px)`
})
</script>

<style lang="postcss" scoped>
.img-scroll {
  position: absolute;
  left: -100px;
  opacity: 0.2;
  display: flex;
  img {
    margin-right: 20px;
  }
}
</style>
