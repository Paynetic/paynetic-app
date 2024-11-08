<template>
  <div class="home-clients-wrap">
    <div class="clients-left f-col">
      <div class="testimonials">
        {{ ts('testimonials') }}
      </div>
      <div class="clients-title">
        {{ ts('clients_title') }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { store } from '@app/store'
import { ts } from '../../i18n'
</script>

<style lang="postcss" setup>
@import '../../css/defines.postcss';

.home-clients-wrap {
  padding: 40px 20px 40px 20px;
  position: relative;
  margin-top: 140px;
  background-color: $primary;
}
.testimonials {
  @mixin medium 16px;
  color: white;
  letter-spacing: 0.35em;
}
.clients-title {
  @mixin title 36px;
  color: white;
}

@media (max-width: 1180px) {
}
@media (max-width: 800px) {
}
@media (max-width: 800px) {
}
</style>
