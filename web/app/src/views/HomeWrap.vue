<template>
  <HomeBg />
  <HomeHeader />
  <div id="scroll-container" class="app-content">
    <router-view class="app-router-view" />
    <PNFooter />
  </div>
</template>

<script lang="ts" setup>
import HomeHeader from '../components/nav/HomeHeader.vue'
import PNFooter from '../components/nav/PNFooter.vue'
import HomeBg from '../components/home/HomeBg.vue'
</script>

<style lang="postcss" scoped></style>
