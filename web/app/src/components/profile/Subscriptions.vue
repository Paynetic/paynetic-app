<template>
  <div class="subscriptions-wrap">
    <div class="no-subscriptions f-center-col">
      <div class="no-text">
        {{ ts('profile.no_subscriptions') }}
      </div>
      <router-link :to="{ name: 'Browse' }" class="browse-link">
        <PNButton :text="ts('footer.browse')" class="button2" />
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

.subscriptions-wrap {
  min-height: 240px;
  padding: 0 24px;
}
.no-subscriptions {
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
