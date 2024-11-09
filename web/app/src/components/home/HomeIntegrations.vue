<template>
  <div class="home-integrations-wrap">
    <Scroller :width="484" :img="IntegrationsScroll" class="scroll" />
    <div class="integrations-left f-center-col">
      <div class="integrations">
        {{ ts('integrations') }}
      </div>
      <div class="integrations-title">
        {{ ts('integrations_title') }}
      </div>
      <div class="integrations-text">
        {{ ts('integrations_text') }}
      </div>
      <IntegrationTypes />
      <router-link :to="{ name: 'Integration' }" class="supported">
        {{ ts('supported') }}
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ts } from '../../i18n'
import IntegrationsScroll from '../../assets/img/integrations-scroll.svg'
import Scroller from '../widgets/Scroller.vue'
import IntegrationTypes from './IntegrationTypes.vue'
</script>

<style lang="postcss" setup>
@import '../../css/defines.postcss';

.home-integrations-wrap {
  padding: 80px 40px 104px 40px;
  position: relative;
  margin-top: 140px;
  background-color: $dark-bg;
  text-align: center;
}
.scroll {
  opacity: 0.2;
  top: 68px;
}
.integrations {
  @mixin medium 16px;
  text-transform: uppercase;
  letter-spacing: 0.35em;
  color: $text4;
}
.integrations-title {
  @mixin extra 36px;
  color: white;
  margin-top: 16px;
}
.integrations-text {
  @mixin text 18px;
  color: white;
  margin-top: 20px;
}

.supported {
  @mixin bold 18px;
  color: white;
  margin-top: 60px;
  border: 1px solid white;
  padding: 15px 25px;
  transition: all 0.25s ease;
  &:hover {
    color: $dark-bg;
    background-color: white;
  }
}
</style>
