<template>
  <div class="product-top">
    <div class="image-wrap">
      <img :src="getMainImage(product)" />
    </div>
    <div class="top-detail f-col">
      <div class="name">
        {{ product.name }}
      </div>
      <div class="blurb">
        {{ product.blurb }}
      </div>
      <div class="info1">
        <div class="purchases">
          {{ `0 sold` }}
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { IProductViewModel } from '@app/types'
import { getMainImage } from '@app/util'

const { product } = defineProps<{
  product: IProductViewModel
}>()
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.product-top {
  display: flex;
  padding-bottom: 40px;
  border-bottom: 1px solid $border1;
}
.image-wrap {
  position: relative;
  width: 55%;
  background-color: #dee2e4;
  height: 280px;
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}
.top-detail {
  padding-left: 24px;
}
.name {
  @mixin bold 36px;
}
.blurb {
  @mixin medium 18px;
  margin-top: 12px;
}
.info1 {
  @mixin text 19px;
  display: flex;
  margin-top: 20px;
}
.purchases {
  padding-right: 12px;
  border-right: 1px solid $text-light2;
}

@media (max-width: 800px) {
  .product-top {
    flex-direction: column;
  }
  .image-wrap {
    margin: 0 auto;
    width: 100%;
    max-width: 560px;
  }
  .top-detail {
    margin: 16px auto 0;
    width: 100%;
    max-width: 560px;
  }
}
</style>
