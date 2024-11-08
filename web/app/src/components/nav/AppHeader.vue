<template>
  <header>
    <div class="header">
      <router-link :to="{ name: 'Home' }">
        <Logo class="header-logo" />
      </router-link>
      <div class="header-links">
        <HeaderLink to="Integration" :text="ts('integration.title')" />
        <HeaderLink to="Features" :text="ts('features.title')" />
        <HeaderLink to="Browse" :text="ts('browse')" />
      </div>
      <PNUserMenu v-if="loggedIn" class="user-menu" />
      <router-link v-else :to="{ name: 'Connect' }" class="start-link">
        <div class="join button1">
          {{ ts('connect.join') }}
        </div>
      </router-link>
      <Burger @click="showDrawer(true)" />
      <Drawer :active="drawerActive" @close="showDrawer(false)" class="header-drawer">
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
        <DrawerLink to="Browse" :text="ts('browse')" @click="showDrawer(false)" />
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
import { drawerActive, showDrawer, loggedIn } from '@app/features'
import { ts } from '../../i18n'
import Burger from '../widgets/Burger.vue'
import Drawer from './Drawer.vue'
import DrawerLink from './DrawerLink.vue'
import HeaderLink from './HeaderLink.vue'
import PNUserMenu from './PNUserMenu.vue'
import Logo from '../svg/Logo.vue'
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.join {
  margin-left: 24px;
}
.user-menu {
  margin-left: auto;
}
.burger {
  display: none;
  margin-left: 16px;
}
.header-drawer {
  display: none;
}
@media (max-width: 860px) {
  .header {
    padding-top: 40px;
  }
  .header-logo {
    width: 140px;
  }
  .links a {
    margin: 0 12px;
  }
}
@media (max-width: 680px) {
  .links {
    display: none;
  }
  .header-drawer {
    display: block;
  }
  .start-link {
    margin-left: auto;
    margin-right: 16px;
  }
  .burger {
    display: flex;
  }
}
</style>
