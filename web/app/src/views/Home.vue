<template>
  <div class="home-wrap">
    <HomeHero />
    <HomeWhy />
    <HomeHow />
    <!--<HomeClients />-->
    <HomeIntegrations />
    <HomeScale />
  </div>
</template>

<script lang="ts" setup>
import HomeHero from '../components/home/HomeHero.vue'
// import HomeClients from '../components/home/HomeClients.vue'
import HomeIntegrations from '../components/home/HomeIntegrations.vue'
import HomeScale from '../components/home/HomeScale.vue'
import HomeHow from '../components/home/HomeHow.vue'
import HomeWhy from '../components/home/HomeWhy.vue'
</script>

<style lang="postcss" scoped></style>
