<template>
  <div class="products-wrap">
    <div class="no-products f-center-col">
      <div class="no-text">
        {{ ts('product.empty') }}
      </div>
      <router-link :to="{ name: 'Create' }" class="browse-link">
        <PNButton :text="ts('profile.create')" class="button2" />
      </router-link>
    </div>
  </div>
</template>

<script lang="ts" setup>
import PNButton from '../widgets/PNButton.vue'
import { ts } from '../../i18n'
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.products-wrap {
  min-height: 240px;
  padding: 0 24px;
}
.no-products {
  padding: 40px 0 0 0;
}
.no-text {
  @mixin semibold 18px;
  color: $text3;
  text-align: center;
}
.browse-link {
  margin-top: 24px;
}
</style>
