<template>
  <div class="scale-wrap">
    <img class="scale-img overlay" src="../../assets/img/scale-bg.jpg" />
    <div class="scale-content f-center-col">
      <div class="scale-title">
        {{ ts('scale.ready') }}
      </div>
      <div class="scale-text">
        {{ ts('scale.text') }}
      </div>
      <router-link :to="{ name: 'Connect' }">
        <div class="scale-join">
          {{ ts('scale.start') }}
        </div>
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ts } from '../../i18n'
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.scale-wrap {
  position: relative;
  height: 602px;
}
.scale-content {
  position: relative;
  height: 100%;
  padding: 0 40px;
  max-width: 90%;
  margin: 0 auto;
}
.scale-title {
  @mixin bold 36px;
  color: $text2;
  max-width: 800px;
  text-align: center;
}
.scale-divider {
  background-color: white;
  width: 114px;
  height: 3px;
  margin: 14px 0 16px;
}
.scale-text {
  @mixin text 18px;
  max-width: 800px;
  text-align: center;
  margin-top: 16px;
}
.scale-join {
  @mixin bold 18px;
  display: flex;
  align-items: center;
  background-color: $primary;
  transition: all 0.25s ease;
  cursor: pointer;
  height: 56px;
  padding: 0 24px;
  margin-top: 40px;
  color: white;
  &:hover {
    color: white;
    background-color: black;
  }
}
@media (max-width: 600px) {
  .scale {
    max-width: 360px;
    height: 500px;
  }
  .scale-join {
    height: 43px;
    font-size: 16px;
    padding: 0 20px;
  }
}
</style>
