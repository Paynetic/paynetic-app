<template>
  <div class="profile-wrap">
    <ProfileTop @edit="showEditModal = true" />
    <RouterTabView :routes="routes" variant="link" class="profile-router-tabs" />
    <EditProfileModal :show="showEditModal" @cancel="showEditModal = false" />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { IRoute } from '@app/types'
import RouterTabView from '../components/nav/RouterTabView.vue'
import ProfileTop from '../components/profile/ProfileTop.vue'
import { ts } from '../i18n'
import EditProfileModal from '../components/profile/EditProfileModal.vue'

const routes: IRoute[] = [
  {
    name: 'Subscriptions',
    label: ts('subscriptions'),
    labelClass: 'subscriptions-tab',
  },
  {
    name: 'Products',
    label: ts('products'),
    labelClass: 'products-tab',
  },
  {
    name: 'Settings',
    label: ts('profile.settings'),
    labelClass: 'settings-tab',
  },
]

const showEditModal = ref(false)
</script>

<style lang="postcss" scoped>
.profile-wrap {
  padding-top: 140px;
  padding-bottom: 80px;
}
.profile-content {
  min-height: 240px;
}
.profile-router-tabs {
  max-width: 1040px;
  margin: 40px auto 0;
  padding: 0 40px;
  :deep(.router-tab-view-tabs) {
    margin-bottom: 24px;
    position: relative;
  }
  &.hide {
    :deep(.router-tab-view-tabs) {
      display: none;
    }
  }
}
@media (max-width: 600px) {
  .profile-router-tabs {
    padding: 0;
    :deep(.router-tab-view-tabs) {
      justify-content: center;
    }
    :deep(.router-tab-view-tab) {
      &.router-tab-view-tab--link {
        margin-right: 16px;
        font-size: 15px;
      }
    }
  }
}
</style>
