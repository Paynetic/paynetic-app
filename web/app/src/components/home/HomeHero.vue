<template>
  <div class="hero-wrap">
    <img class="hero-bg overlay" :src="HeroBg" />
    <div class="home-hero f-col">
      <h1 class="title" v-html="ts('seamless')"></h1>
      <div class="divider"></div>
      <div class="home-text" v-html="ts('effortless')"></div>
      <svg viewBox="0 0 20 18" xmlns="http://www.w3.org/2000/svg" class="hero-arrow">
        <g fill="#fff">
          <path
            d="m17.101 7.908-7.544 7.616-7.543-7.616c-.449-.437-1.162-.431-1.602.015-.441.445-.447 1.164-.014 1.617l7.756 7.831c.773.779 2.024.779 2.798 0l7.756-7.831c.389-.457.365-1.139-.056-1.567-.421-.427-1.097-.455-1.551-.065z"
          />
          <path
            d="m8.158 9.846c.773.778 2.025.778 2.798 0l7.756-7.831c.405-.456.387-1.152-.042-1.585-.428-.434-1.117-.454-1.569-.047l-7.544 7.615-7.543-7.615c-.287-.3-.712-.42-1.112-.315-.399.106-.711.421-.816.824-.105.404.015.833.312 1.123z"
          />
        </g>
      </svg>
    </div>
    <Scroller :width="400" :img="ScrollImg" class="hero-scroller" />
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../../i18n'
import HeroBg from '../../assets/img/hero-bg.jpg'
import ScrollImg from '../../assets/img/paynetic-scroll.svg'
import Scroller from '../widgets/Scroller.vue'
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.hero-wrap {
  background-color: $primary;
  position: relative;
}
.home-hero {
  padding: 200px 40px 320px 40px;
  max-width: 1200px;
  height: 100vh;
  min-height: 800px;
  max-height: 960px;
  margin: 0 auto;
  display: flex;
  position: relative;
  color: white;
  align-items: center;
}
.hero-bg {
  object-fit: cover;
}
.hero-scroller {
  bottom: 0;
}
.hero-arrow {
  width: 20px;
  height: 18px;
  opacity: 0.5;
  flex-shrink: 0;
  position: relative;
  top: 0;
  user-select: none;
  transition: all 0.25s ease;
  margin-top: 40px;
  &:hover {
    opacity: 1;
    top: 4px;
  }
}

.title {
  @mixin thin 54px;
  font-weight: 300;
  max-width: 800px;
  line-height: 1.2em;
  text-align: center;
  :deep(span) {
    font-style: italic;
  }
}
.divider {
  width: 175px;
  height: 1px;
  background-color: white;
}
.home-text {
  @mixin text 18px;
  text-align: center;
  max-width: 610px;
  margin-top: 20px;
  :deep(span) {
    font-weight: 700;
    font-style: italic;
  }
}
@media (max-width: 1000px) {
  .title {
    font-size: 48px;
  }
}
@media (max-width: 800px) {
  .home-hero {
    flex-direction: column;
    align-items: center;
  }
  .title {
    justify-content: center;
  }
  .buttons {
    justify-content: center;
  }
}
</style>
