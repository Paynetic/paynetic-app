<template>
  <div class="how-integrate">
    <img src="../../assets/img/circle-dots.svg" class="circle-dots" />
    <div class="front overlay f-center">
      <div class="button-wrap">
        <div class="button">
          <div class="button-content">
            <div class="subscribe">
              {{ ts('subscribe_with') }}
            </div>
            <Logo class="header-logo" />
          </div>
        </div>
        <img src="../../assets/img/plus.svg" class="plus" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../../i18n'
import Logo from '../svg/Logo.vue'
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.how-integrate {
  width: 100%;
  height: 100%;
}
.circle-dots {
  height: 60%;
  margin: 10% 0 0 40px;
}
.button-wrap {
  position: relative;
  width: 60%;
}
.button {
  padding: 28px 24px;
  background-color: $purple;
  border-radius: 12px;
}
.subscribe {
  @mixin bold 12px;
  color: #c7c8ca;
}
.header-logo {
  margin-top: 4px;
}
.plus {
  width: 76px;
  position: absolute;
  bottom: -44px;
  right: -24px;
}
</style>
