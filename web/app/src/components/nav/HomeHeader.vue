<template>
  <header>
    <div class="header">
      <router-link :to="{ name: 'Home' }">
        <Logo class="header-logo" />
      </router-link>
      <div class="header-links">
        <HeaderLink to="About" :text="ts('footer.about')" />
        <HeaderLink to="Integration" :text="ts('integration.title')" />
        <HeaderLink to="Features" :text="ts('features.title')" />
        <HeaderLink to="Faq" :text="ts('footer.faq')" />
      </div>
      <PNUserMenu v-if="loggedIn" class="user-menu" />
      <router-link v-else :to="{ name: 'Connect' }" class="start-link">
        <div class="start button1">
          {{ ts('connect.start') }}
        </div>
      </router-link>
      <Burger @click="showDrawer(true)" />
      <Drawer :active="drawerActive" @close="showDrawer(false)" class="header-drawer">
        <DrawerLink to="About" :text="ts('footer.about')" @click="showDrawer(false)" />
        <DrawerLink
          to="Integration"
          :text="ts('integration.title')"
          @click="showDrawer(false)"
        />
        <DrawerLink
          to="Features"
          :text="ts('features.title')"
          @click="showDrawer(false)"
        />
        <DrawerLink to="Faq" :text="ts('footer.faq')" @click="showDrawer(false)" />
        <router-link
          :to="{ name: 'Connect' }"
          class="drawer-start"
          @click="showDrawer(false)"
        >
          {{ ts('connect.start') }}
        </router-link>
      </Drawer>
    </div>
  </header>
</template>

<script lang="ts" setup>
import { drawerActive, showDrawer } from '@app/features'
import { ts } from '../../i18n'
import Logo from '../svg/Logo.vue'
import Burger from '../widgets/Burger.vue'
import Drawer from './Drawer.vue'
import DrawerLink from './DrawerLink.vue'
import HeaderLink from './HeaderLink.vue'
import { loggedIn } from '@app/features'
import PNUserMenu from './PNUserMenu.vue'
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.start {
  margin-left: 24px;
  flex-shrink: 0;
  width: 121px;
}
@media (max-width: 880px) {
  .start {
    margin-left: 16px;
  }
}
</style>
