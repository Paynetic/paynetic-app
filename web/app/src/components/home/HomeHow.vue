<template>
  <div class="home-how f-center-col">
    <Scroller :width="499" :img="HowScroll" class="how-scroll" />
    <div class="how-it">
      {{ ts('how_it') }}
    </div>
    <div class="how-title">
      {{ ts('how_title') }}
    </div>
    <div class="how-items">
      <img class="item-bg overlay" :src="HowBg" />
      <div class="item-left f-center-col">
        <div class="item-title">
          {{ activeItem.title }}
        </div>
        <div class="item-text">
          {{ activeItem.text }}
        </div>
      </div>
      <component :is="activeItem.right" class="item-right" />
    </div>
    <div class="how-steps">
      <div class="step" :class="{ active: activeIndex === 0 }" @click="select(0)">
        <div class="step-text">
          {{ ts('step') }}
        </div>
        <div class="step-num">01</div>
      </div>
      <div class="step-line"></div>
      <div class="step" :class="{ active: activeIndex === 1 }" @click="select(1)">
        <div class="step-text">
          {{ ts('step') }}
        </div>
        <div class="step-num">02</div>
      </div>
      <div class="step-line"></div>
      <div class="step" :class="{ active: activeIndex === 2 }" @click="select(2)">
        <div class="step-text">
          {{ ts('step') }}
        </div>
        <div class="step-num">03</div>
      </div>
    </div>
    <router-link :to="{ name: 'Integration' }" class="learn-more">
      {{ ts('integration_learn') }}
    </router-link>
  </div>
</template>

<script lang="ts" setup>
import { computed, markRaw, onMounted, onUnmounted, ref } from 'vue'
import { ts } from '../../i18n'
import HowScroll from '../../assets/img/how-scroll.svg'
import HowBg from '../../assets/img/how-bg.jpg'
import HowConnect from './HowConnect.vue'
import HowAutomate from './HowAutomate.vue'
import HowIntegrate from './HowIntegrate.vue'
import Scroller from '../widgets/Scroller.vue'

let howInterval: ReturnType<typeof setInterval> | undefined

const howItems = [
  {
    title: ts('integrate'),
    text: ts('integrate_text'),
    right: markRaw(HowIntegrate),
  },
  {
    title: ts('connect_wallets'),
    text: ts('how_connect'),
    right: markRaw(HowConnect),
  },
  {
    title: ts('automate_title'),
    text: ts('automate_text'),
    right: markRaw(HowAutomate),
  },
]

const activeIndex = ref(1)

const activeItem = computed(() => {
  return howItems[activeIndex.value]
})

const clear = () => {
  if (howInterval) {
    clearInterval(howInterval)
    howInterval = undefined
  }
}
const start = () => {
  howInterval = setInterval(() => {
    activeIndex.value = (activeIndex.value + 1) % 3
  }, 2500)
}

const select = (index: number) => {
  clear()
  activeIndex.value = index
  start()
}

onMounted(() => {
  start()
})

onUnmounted(() => {
  clear()
})
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.home-how {
  background-color: #b5d8cd40;
  position: relative;
  padding: 72px 0 100px;
  text-align: center;
}
.how-scroll {
  opacity: 0.45;
  top: 56px;
}
.how-it {
  @mixin medium 16px;
  color: $text3;
  letter-spacing: 0.35em;
  text-transform: uppercase;
}
.how-title {
  @mixin extra 36px;
  margin-top: 8px;
  padding: 0 40px;
  color: $purple;
}
.how-items {
  display: flex;
  position: relative;
  filter: drop-shadow(rgba(204, 204, 204, 0.25) -2px 2px 7px);
  height: 360px;
  width: 100%;
  max-width: 1010px;
  margin-top: 40px;
}
.item-bg {
  object-fit: cover;
}
.item-left {
  position: relative;
  align-items: flex-start;
  padding: 0 32px 0 40px;
  width: 50%;
}
.item-right {
  position: relative;
  background-color: $text3;
  width: 50%;
}
.item-title {
  @mixin bold 36px;
  color: $text2;
}
.item-text {
  @mixin text 18px;
  margin-top: 12px;
}
.how-steps {
  margin-top: 32px;
  display: flex;
  align-items: center;
  text-align: center;
}
.step {
  color: rgb(224, 224, 224);
  cursor: pointer;
  user-select: none;
  &.active {
    color: rgb(191, 191, 191);
  }
}
.step-text {
  @mixin title 15px;
  letter-spacing: 0.25em;
}
.step-num {
  @mixin title 36px;
}
.step-line {
  width: 152px;
  height: 1px;
  margin: 0 24px;
  background-color: rgb(222, 222, 222);
}
.learn-more {
  @mixin thin 20px;
  color: $text3;
  margin-top: 40px;
}
@media (max-width: 800px) {
  .home-how {
    padding: 180px 0 140px;
  }
  .how-scroll {
    top: 160px;
  }
  .how-items {
    flex-direction: column;
    width: 100%;
    max-width: 520px;
    height: auto;
  }
  .item-left {
    width: 100%;
    height: 280px;
    align-items: center;
  }
  .item-right {
    width: 100%;
    height: 320px;
  }
  .step-line {
    width: 40px;
    margin: 0 16px;
  }
}
</style>
