<template>
  <footer>
    <div class="footer-wrap">
      <img :src="FooterBg" class="footer-bg overlay" />
      <div class="footer-content f-col">
        <div class="footer-top">
          <div class="footer-left">
            <div class="footer-title">
              {{ ts('footer.title') }}
            </div>
            <div class="footer-text">
              {{ ts('footer.text') }}
            </div>
          </div>
          <div class="footer-cols">
            <div class="footer-col">
              <div class="label">{{ ts('footer.links') }}</div>
              <router-link
                :to="{ name: 'Home' }"
                class="link"
                :class="{ active: route.name === 'Home' }"
              >
                {{ ts('home') }}
              </router-link>
              <router-link
                :to="{ name: 'About' }"
                class="link"
                :class="{ active: route.name === 'About' }"
              >
                {{ ts('footer.about') }}
              </router-link>
            </div>
            <div class="footer-col">
              <div class="label empty-label">&nbsp;</div>
              <router-link
                :to="{ name: 'Integration' }"
                class="link"
                :class="{ active: route.name === 'Integration' }"
              >
                {{ ts('integration.title') }}
              </router-link>
              <router-link
                :to="{ name: 'Features' }"
                class="link"
                :class="{ active: route.name === 'Features' }"
              >
                {{ ts('features.title') }}
              </router-link>
              <router-link
                :to="{ name: 'Connect' }"
                class="link"
                :class="{ active: route.name === 'Connect' }"
              >
                {{ ts('connect.start') }}
              </router-link>
            </div>
            <div class="footer-col">
              <div class="label">{{ ts('footer.resources') }}</div>
              <a href="http://bit.ly/3UAtSp0" class="link" target="_blank">
                {{ ts('footer.whitepaper') }}
              </a>
              <router-link
                :to="{ name: 'Faq' }"
                class="link"
                :class="{ active: route.name === 'Faq' }"
              >
                {{ ts('footer.faq') }}
              </router-link>
            </div>
          </div>
        </div>
        <div class="footer-bottom">
          <div class="footer-left">
            <router-link :to="{ name: 'Home' }">
              <Logo :secondary="true" class="footer-logo" />
            </router-link>
            <div class="socials">
              <a href="https://x.com/PayneticOfficial" target="_blank">
                <div class="social"><Twitter /></div>
              </a>
              <a href="https://t.me/PayneticOfficial" target="_blank">
                <div class="social"><Telegram /></div>
              </a>
              <a href="https://medium.com/@PayneticOfficial" target="_blank">
                <div class="social"><Medium /></div>
              </a>
            </div>
          </div>
          <div class="footer-br f-col">
            <div class="copyright">
              {{ ts('footer.copyright') }}
            </div>
            <div class="bottom-links">
              <router-link
                :to="{ name: 'Trust' }"
                class="bottom-link"
                :class="{ active: route.name === 'Trust' }"
              >
                {{ ts('footer.trust') }}
              </router-link>
              <div class="bottom-divider"></div>
              <router-link
                :to="{ name: 'Terms' }"
                class="bottom-link"
                :class="{ active: route.name === 'Terms' }"
              >
                {{ ts('footer.terms') }}
              </router-link>
              <div class="bottom-divider"></div>
              <router-link
                :to="{ name: 'Privacy' }"
                class="bottom-link"
                :class="{ active: route.name === 'Privacy' }"
              >
                {{ ts('footer.privacy') }}
              </router-link>
              <div class="bottom-divider"></div>
              <router-link
                :to="{ name: 'Cookies' }"
                class="bottom-link"
                :class="{ active: route.name === 'Cookies' }"
              >
                {{ ts('footer.cookie') }}
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { ts } from '../../i18n'
import Logo from '../svg/Logo.vue'
import Medium from '../svg/Medium.vue'
import Telegram from '../svg/Telegram.vue'
import Twitter from '../svg/Twitter.vue'
import FooterBg from '../../assets/img/footer-bg.jpg'

const route = useRoute()
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

$text-color: rgb(130, 206, 199);

.footer {
  position: relative;
}
.bottom-links {
  display: flex;
  margin-top: 8px;
}
.bottom-divider {
  height: 100%;
  width: 1px;
  height: 17px;
  background-color: white;
  margin: 0 20px;
}
.bottom-link {
  @mixin text 12px;
  letter-spacing: 1.2px;
  color: white;
  &:hover {
    color: $text-color;
  }
}
a.active {
  color: $text-color;
}
.footer-wrap {
  position: relative;
  background-color: black;
  color: white;
}
.footer-bg {
  object-fit: cover;
  object-position: 57.2% 23.4%;
}
.footer-content {
  position: relative;
  padding: 120px 40px 120px;
  width: 100%;
  max-width: 1080px;
  margin: 0 auto;
}
.footer-top {
  display: flex;
  justify-content: space-between;
  flex-direction: row;
}
.footer-bottom {
  display: flex;
  justify-content: space-between;
  flex-direction: row;
}
.footer-left {
  max-width: 300px;
}
.footer-title {
  @mixin bold 20px;
  color: $text-color;
}
.footer-text {
  @mixin text 15px;
  margin-top: 8px;
}
.footer-logo {
  width: 220px;
  margin-top: 40px;
}
.footer-br {
  margin-top: 48px;
  align-items: flex-end;
  position: relative;
}
.copyright {
  @mixin bold 13px;
  letter-spacing: 0.03em;
  margin-top: 8px;
}
.footer-cols {
  display: flex;
  margin-left: auto;
}
.footer-col {
  display: flex;
  flex-direction: column;
  margin-left: 64px;
}
.label {
  @mixin title 15px;
  font-weight: 800;
  color: $text3;
  letter-spacing: 0.03em;
  margin-bottom: 12px;
}
.link {
  @mixin text 15px;
  padding: 6px 0;
  color: white;
  &:not(:last-child) {
    border-bottom: 1px solid rgba(181, 216, 205, 0.25);
  }
}
.socials {
  display: flex;
  margin-top: 4px;
}
.social {
  width: 28px;
  height: 28px;
  padding: 4px;
  cursor: pointer;
  margin-right: 8px;
  :deep(svg) {
    fill: white;
    transition: fill 0.25s ease;
    width: 24px;
    height: 24px;
  }
  &:hover {
    :deep(svg) {
      fill: $text-color;
    }
  }
}
@media (max-width: 820px) {
  .footer-content {
    padding: 48px 40px 48px;
  }
  .footer-top {
    flex-direction: column;
    align-items: center;
  }
  .footer-bottom {
    flex-direction: column;
    align-items: center;
  }
  .footer-cols {
    margin-left: 0;
    margin: 48px 0 0 0;
  }
  .footer-col {
    &:first-child {
      margin-left: 0;
    }
  }
  .footer-left {
    max-width: 600px;
    text-align: center;
  }
  .footer-br {
    align-items: center;
    margin-top: 24px;
  }
  .footer-wrap {
    overflow: hidden;
  }
  .footer-bg {
    object-position: 49.9% 8.7%;
    transform: scale(2);
  }
}
@media (max-width: 700px) {
  .footer-bg {
    object-position: 49.9% 0%;
    transform: scale(2);
  }
}
@media (max-width: 700px) {
  .footer-bg {
    object-position: 49.9% -10%;
    transform: scale(2);
  }
}
@media (max-width: 510px) {
  .footer-bg {
    object-position: 49.9% 140px;
    transform: scale(3);
  }
  .footer-cols {
    flex-direction: column;
    margin-top: 24px;
  }
  .footer-col {
    margin-left: 0;
    margin-top: 0;
    text-align: center;
    &:last-child {
      margin-top: 0;
    }
  }
  .label {
    margin-top: 16px;
  }
  .empty-label {
    display: none;
  }
  .footer-left {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .bottom-link {
    text-align: center;
  }
  .bottom-divider {
    margin: 0 10px;
  }
}
@media (max-width: 460px) {
  .footer-bg {
    object-position: 49.9% 220px;
    transform: scale(3);
  }
}
</style>
