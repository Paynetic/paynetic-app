<template>
  <HomeBg />
  <AppHeader />
  <div id="scroll-container" class="app-content">
    <router-view class="app-router-view" />
    <PNFooter />
  </div>
</template>

<script lang="ts" setup>
import HomeBg from '../components/home/HomeBg.vue'
import AppHeader from '../components/nav/AppHeader.vue'
import PNFooter from '../components/nav/PNFooter.vue'
</script>
