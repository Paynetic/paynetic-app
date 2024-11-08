<template>
  <div v-if="state.loading">
    <Spinner :size="40" />
  </div>
  <div v-else-if="state.product" class="product-wrap">
    <ProductTop :product="state.product" />
    <ProductDetail :product="state.product" />
  </div>
  <div v-else class="not-found-wrap">
    <div class="not-found-title">
      {{ ts('not_found') }}
    </div>
    <div class="not-found-text">
      {{ ts('not_found_text') }}
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getProduct, IGetProductParams } from '@app/features'
import ProductTop from '../components/product/ProductTop.vue'
import { onMounted, reactive } from 'vue'
import Spinner from '../components/widgets/Spinner.vue'
import { ts } from '../i18n'
import { useRoute } from 'vue-router'
import ProductDetail from '../components/product/ProductDetail.vue'

const route = useRoute()

const state: IGetProductParams = reactive({
  loading: true,
  error: undefined,
  product: undefined,
})

onMounted(() => {
  if (route.params.id) {
    getProduct(route.params.id as string, state)
  } else {
    state.loading = false
  }
})
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.product-wrap {
  min-height: calc(100vh - 360px);
  max-width: 1040px;
  padding: 96px 24px 120px;
  margin: 0 auto;
}

.not-found-wrap {
  min-height: calc(100vh - 360px);
  padding: 96px 24px 80px;
  max-width: 600px;
  margin: 0 auto;
  text-align: center;
  color: $text3;
}
.not-found-title {
  @mixin bold 32px;
}
.not-found-text {
  @mixin medium 20px;
  margin-top: 24px;
}
</style>
