<template>
  <div class="products-wrap">
    <div class="top-wrap">
      <div class="title-wrap">
        <h1 class="products-title">
          {{ ts('product.title') }}
        </h1>
        <div class="products-text">
          <span>{{ ts('product.text') }}</span>
        </div>
      </div>
      <router-link :to="{ name: 'Create' }" class="create-link">
        <PNButton :text="ts('profile.create')" class="button2" />
      </router-link>
    </div>
    <ProductList :products="state.products" :loading="state.loading" />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, reactive } from 'vue'
import { IListProductParams, listProducts } from '@app/features'
import ProductList from '../components/product/ProductList.vue'
import { ts } from '../i18n'
import PNButton from '../components/widgets/PNButton.vue'

const state: IListProductParams = reactive({
  error: undefined,
  loading: false,
  products: [],
})

onMounted(() => {
  listProducts({}, state)
})
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.products-wrap {
  max-width: 1040px;
  padding: 120px 24px 120px;
  margin: 0 auto;
  min-height: calc(100vh - 314px);
  color: $text3;
}
.products-title {
  @mixin bold 42px;
  margin: 64px 0 16px;
  color: $text4;
}
.top-wrap {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
}
.create-link {
  margin-bottom: 16px;
}

.products-text {
  @mixin text 18px;
  max-width: 460px;
  line-height: 150%;
  a {
    color: $text3;
    text-decoration: underline;
  }
}
</style>
