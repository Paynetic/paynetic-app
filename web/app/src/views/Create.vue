<template>
  <div class="create-wrap">
    <Placeholder v-if="disabled" />
    <div v-else class="create">
      <div class="create-title">
        {{ ts('create.title') }}
      </div>
      <div class="create-text">
        {{ ts('create.text') }}
      </div>
      <PNInput v-model="name" :label="ts('name')" class="edit-input" />
      <PNTextArea v-model="blurb" :label="ts('blurb')" class="edit-input" />
      <PNTextArea v-model="description" :label="ts('description')" class="edit-input" />
      <div class="category"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import Placeholder from '../components/create/Placeholder.vue'
import PNInput from '../components/widgets/PNInput.vue'
import PNTextArea from '../components/widgets/PNTextArea.vue'
import { ts } from '../i18n'

const disabled = true

const name = ref('')
const blurb = ref('')
const description = ref('')
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.create-wrap {
  padding-top: 100px;
  padding-bottom: 80px;
}
.create {
  max-width: 940px;
  padding: 0 32px 120px;
  margin: 0 auto;
  position: relative;
}
.create-title {
  @mixin bold 36px;
}
.create-text {
  @mixin text 18px;
}
.edit-input {
  margin-top: 16px;
}
</style>
