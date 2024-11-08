<template>
  <div class="connect-welcome f-col">
    <h1 class="connect-title" v-html="ts('connect.title')"></h1>
    <div class="connect-text">
      {{ ts('connect.text') }}
    </div>
    <PNButton
      :text="ts('connect.button')"
      :animate="loadingAccount"
      class="button1 button3"
      @click="emit('connect')"
    />
    <ErrorMessage :error="errorKey && ts(errorKey)" class="connect-error" />
    <div class="email-login" @click="emit('emailLogin')">
      {{ ts('connect.email_login') }}
    </div>
    <div class="how">
      {{ ts('connect.how') }}
    </div>
    <ol class="how-items">
      <li v-html="ts('connect.install')"></li>
      <li>{{ ts('connect.click') }}</li>
      <li>{{ ts('connect.email') }}</li>
    </ol>
  </div>
</template>

<script setup lang="ts">
import { ts } from '../../i18n'
import PNButton from '../widgets/PNButton.vue'
import ErrorMessage from '../widgets/ErrorMessage.vue'

defineProps<{
  loadingAccount: boolean
  errorKey: string | undefined
}>()

const emit = defineEmits<{
  (e: 'connect'): void
  (e: 'emailLogin'): void
}>()
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.connect-welcome {
  align-items: center;
  color: white;
}

.button3 {
  width: 120px;
  margin: 40px auto 0;
  text-align: center;
}
.how {
  @mixin title 20px;
  margin-top: 48px;
  text-align: center;
}
.connect-error {
  margin-top: 0px;
}
.email-login {
  @mixin bold 15px;
  color: $text4;
  margin-top: 20px;
  cursor: pointer;
  user-select: none;
}
.how-items {
  @mixin text 18px;
  max-width: 480px;
  margin: 8px auto;
  li {
    margin-top: 12px;
  }
  :deep(a) {
    color: $text4;
  }
}
</style>
