<template>
  <router-link v-if="product" :to="{ name: 'Product', params: { id: product.id } }">
    <div class="product-card">
      <div class="image-wrap">
        <img :src="getMainImage(product)" />
        <div class="image-overlay overlay f-center">
          <div class="button1 view-button">
            {{ ts('view') }}
          </div>
        </div>
        <div class="active-wrap">
          <div class="active">
            {{ product.status }}
          </div>
          <div class="dot" :class="product.status"></div>
        </div>
      </div>
      <div class="product-content">
        <div class="product">
          <h3 class="product-name">
            {{ product.name }}
          </h3>
          <div class="product-blurb">
            {{ product.blurb }}
          </div>
        </div>
        <div class="sold">
          {{ `0 sold` }}
        </div>
      </div>
    </div>
  </router-link>
  <div v-else class="product-card empty">
    <div class="image-wrap"></div>
    <div class="product-content"></div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { IProductViewModel } from '@app/types'
import { toEthDisplay } from '@samatech/vue3-eth'
import { ts } from '../../i18n'
import { getMainImage } from '@app/util'

const { product } = defineProps<{
  product: IProductViewModel | undefined
}>()
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.image-overlay {
  background-color: rgba(200, 200, 200, 0.5);
  opacity: 0;
  transition: opacity 0.25s ease;
}

.active {
  color: $text3;
  opacity: 0;
  transition: opacity 0.25 ease;
}
.active-wrap {
  @mixin medium 13px;
  position: absolute;
  right: 8px;
  top: 6px;
  display: flex;
  align-items: center;
}

.product-card {
  position: relative;
  cursor: pointer;
  color: $text1;
  &:hover {
    .image-overlay {
      opacity: 1;
    }
    .active {
      opacity: 1;
    }
  }
  &.empty {
    opacity: 0;
  }
}
.image-wrap {
  height: 220px;
  background-color: #bec2c4;
  position: relative;
  border-radius: 4px;
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 4px;
  }
}
.view-button {
  color: $text3;
  background-color: white;
}

.product-content {
  display: flex;
  justify-content: space-between;
  padding: 0 6px;
  height: 140px;
  width: 100%;
}
.product-name {
  @mixin semibold 18px;
  margin: 8px 0 4px;
  color: $text4;
}

.product-blurb {
  @mixin text 15px;
  color: $border1;
}
.dot {
  @mixin size 12px;
  border-radius: 50%;
  background-color: $disabled1;
  border: 1.5px solid white;
  margin-left: 4px;
  &.Active {
    background-color: $text3;
  }
}
.sold {
  @mixin medium 13px;
  margin-top: 8px;
  display: flex;
  color: $text3;
  flex-shrink: 0;
}
</style>
