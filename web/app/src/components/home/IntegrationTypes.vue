<template>
  <div class="integration-types">
    <div class="type">
      <div class="type-label">
        {{ ts('commerce') }}
      </div>
      <img v-if="light" src="../../assets/img/woocommerce2.png" class="woocommerce" />
      <img v-else src="../../assets/img/woocommerce.png" class="woocommerce" />
      <img v-if="light" src="../../assets/img/shopify2.png" class="shopify" />
      <img v-else src="../../assets/img/shopify.png" class="shopify" />
    </div>
    <div class="type type2">
      <div class="type-label">
        {{ ts('wallets') }}
      </div>
      <img v-if="light" src="../../assets/img/metamask2.png" class="metamask" />
      <img v-else src="../../assets/img/metamask.png" class="metamask" />
      <img v-if="light" src="../../assets/img/coinbase2.png" class="coinbase" />
      <img v-else src="../../assets/img/coinbase.png" class="coinbase" />
      <img v-if="light" src="../../assets/img/walletconnect2.png" class="walletconnect" />
      <img v-else src="../../assets/img/walletconnect.png" class="walletconnect" />
    </div>
    <div class="type">
      <div class="type-label">
        {{ ts('tools') }}
      </div>
      <img v-if="light" src="../../assets/img/zapier2.png" class="zapier" />
      <img v-else src="../../assets/img/zapier.png" class="zapier" />
      <img v-if="light" src="../../assets/img/hubspot2.png" class="hubspot" />
      <img v-else src="../../assets/img/hubspot.png" class="hubspot" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../../i18n'

defineProps<{
  light?: boolean
}>()
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.integration-types {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  margin-top: 32px;
  color: $text4;
}
.type {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 31px;
  box-shadow: rgba(0, 0, 0, 0.05) 0px 0px 0px 0px;
  background-color: #82cfc70d;
  width: 300px;
  height: 340px;
  margin-top: 16px;
  img {
    margin-top: 44px;
  }
}
.type2 {
  margin: 16px 20px 0;
}
.type-label {
  @mixin bold 20px;
  width: 100%;
  text-align: center;
  padding-bottom: 8px;
  border-bottom: 1px solid #88a7a7;
}
img.woocommerce {
  width: 191px;
  margin-top: 32px;
}
.shopify {
  width: 179px;
}
img.metamask {
  width: 214px;
  margin-top: 32px;
}
.coinbase {
  width: 191px;
}
.walletconnect {
  width: 215px;
}
img.zapier {
  width: 143px;
  margin-top: 32px;
}
.hubspot {
  width: 167px;
}

@media (max-width: 720px) {
  .type {
    max-width: 400px;
    width: 100%;
    height: 320px;
    img {
      margin-top: 32px;
    }
  }
  .type2 {
    margin: 16px 0 0;
  }
}
</style>
