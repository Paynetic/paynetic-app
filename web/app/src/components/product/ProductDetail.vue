<template>
  <div class="product-detail">
    <div class="detail">
      <div class="detail-title">
        {{ ts('product.detail') }}
      </div>
      <div class="detail-text">
        {{ product.description }}
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { IProductViewModel } from '@app/types'
import { ts } from '../../i18n'

defineProps<{
  product: IProductViewModel
}>()
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.product-detail {
  display: flex;
  padding-top: 32px;
}
.detail {
  width: 80%;
  padding: 0 8px 0 4px;
}
.detail-title {
  @mixin semibold 32px;
}
.detail-text {
  @mixin text 18px;
  line-height: 1.5em;
  margin-top: 16px;
}
</style>
