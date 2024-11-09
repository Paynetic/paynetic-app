<template>
  <div class="home-why f-col">
    <div class="home-why-bg"></div>
    <div class="hero-sections">
      <div class="hero-left hero-section">
        <div class="image-wrap">
          <img :src="Hero1" class="hero1" />
        </div>
        <div class="subscription f-col">
          <div class="amount-wrap">
            <div class="amount-text f-col">
              <div class="label">
                {{ ts('monthly') }}
              </div>
              <div class="amount">$9.99 / mo</div>
            </div>
            <img :src="PayDots" class="pay-dots" />
          </div>
          <div class="divider"></div>
          <div class="subscribe-wrap">
            <div class="subscribe-button f-center-col">
              <div class="subscribe-with">
                {{ ts('subscribe_with') }}
              </div>
              <Logo class="subscribe-img" />
            </div>
          </div>
        </div>
        <div class="left-content">
          <div class="left-title">
            {{ ts('simplified') }}
          </div>
          <div class="left-text">
            {{ ts('automate_title') }}
          </div>
        </div>
      </div>
      <div class="hero-right hero-section">
        <img class="get-started overlay" :src="GetStarted" />
        <div class="right-content f-col">
          <div class="gs-title">
            {{ ts('get_started') }}
          </div>
          <div class="gs-text">
            {{ ts('transform') }}
          </div>
          <router-link :to="{ name: 'Connect' }" class="button1 button2 gs-button">
            {{ ts('sign_up') }}
          </router-link>
        </div>
      </div>
    </div>
    <div class="why-items">
      <Scroller :width="339" :img="FeaturesScroll" class="features-scroll" />
      <div class="item-wrap">
        <div class="why-item why1">
          <div class="why-features">
            {{ ts('features.title') }}
          </div>
          <div class="why-title">
            {{ ts('why') }}
          </div>
        </div>
        <div class="border-h h1"></div>
      </div>
      <div class="border-v v1"></div>
      <div class="item-wrap">
        <div class="why-item why2">
          <WhyContent :img="Why1" :title="ts('why_title1')" :text="ts('why_text1')" />
        </div>
        <div class="border-h h2"></div>
      </div>
      <div class="border-v v2"></div>
      <div class="item-wrap">
        <div class="why-item why3">
          <WhyContent :img="Why2" :title="ts('why_title2')" :text="ts('why_text2')" />
        </div>
        <div class="border-h h3"></div>
      </div>
      <div class="border-v v3"></div>
      <div class="item-wrap">
        <div class="why-item why4">
          <WhyContent :img="Why3" :title="ts('why_title3')" :text="ts('why_text3')" />
        </div>
        <div class="border-h h4"></div>
      </div>
      <div class="border-v v4"></div>
      <div class="item-wrap">
        <div class="why-item why5">
          <WhyContent :img="Why4" :title="ts('why_title4')" :text="ts('why_text4')" />
        </div>
        <div class="border-h h5"></div>
      </div>
      <div class="border-v v5"></div>
      <div class="item-wrap">
        <div class="why-item why6">
          <WhyContent :img="Why5" :title="ts('why_title5')" :text="ts('why_text5')" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../../i18n'
import GetStarted from '../../assets/img/get-started.jpg'
import Hero1 from '../../assets/img/hero1.png'
import PayDots from '../../assets/img/pay-dots.png'
import FeaturesScroll from '../../assets/img/features-scroll.svg'
import Why1 from '../../assets/img/why1.svg'
import Why2 from '../../assets/img/why2.svg'
import Why3 from '../../assets/img/why3.svg'
import Why4 from '../../assets/img/why4.svg'
import Why5 from '../../assets/img/why5.svg'
import Logo from '../svg/Logo.vue'
import Scroller from '../widgets/Scroller.vue'
import WhyContent from './WhyContent.vue'
</script>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.home-why {
  position: relative;
  align-items: center;
}
.home-why-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 325px;
  background: linear-gradient(
    180deg,
    rgba(182, 217, 206, 0) 0%,
    rgba(182, 217, 206, 0.5) 31.593116554054053%,
    rgba(255, 255, 255, 0) 100%
  );
}
.features-scroll {
  top: 48px;
  opacity: 0.35;
}
.why-features {
  @mixin medium 16px;
  letter-spacing: 0.35em;
  color: $text3;
  margin-top: 48px;
  text-transform: uppercase;
}
.why-title {
  @mixin extra 36px;
  line-height: 1em;
  color: $purple;
  max-width: 170px;
  margin-top: 12px;
  position: relative;
}
.why-items {
  display: flex;
  position: relative;
  flex-wrap: wrap;
  max-width: 902px;
  margin: 300px auto 120px auto;
}
.item-wrap {
  display: flex;
  flex-direction: column;
  width: 300px;
  height: 250px;
}
.why-item {
  width: 100%;
  height: 100%;
  padding: 16px 20px 16px 28px;
}
.border-h {
  height: 1px;
  position: relative;
  left: 16px;
  background-color: $highlight;
  width: calc(100% - 32px);
}
.border-v {
  position: relative;
  top: 12px;
  width: 1px;
  height: 226px;
  background-color: $highlight;
}
.v3,
.h4,
.h5 {
  display: none;
}

.hero-sections {
  display: flex;
  max-width: 960px;
  width: 100%;
  margin-top: 32px;
  align-items: stretch;
  position: absolute;
  top: -180px;
}
.hero-left {
  display: flex;
  flex-grow: 1;
  margin-right: 2%;
  background-color: #b5d8cd;
  align-items: flex-end;
}
.hero-section {
  position: relative;
  transform: translateY(0px);
  transition: transform 0.25s ease;
  &:hover {
    transform: translateY(-30px);
  }
}
.image-wrap {
  position: absolute;
  left: 28px;
  top: -80px;
  width: 409px;
  aspect-ratio: 1.432;
  filter: blur(1px);
  mask: linear-gradient(0deg, rgba(0, 0, 0, 0) 5%, rgba(0, 0, 0, 1) 70%) add;
}
.hero1 {
  width: 100%;
  height: 100%;
}
.hero-right {
  width: 279px;
  background-color: #b8238d;
  color: white;
  padding: 48px 0;
  flex-shrink: 0;
}
.get-started {
  object-fit: cover;
}
.left-content {
  padding: 0 40px 40px;
  color: $text2;
}
.left-title {
  @mixin bold 36px;
  line-height: 1.1em;
}
.left-text {
  @mixin text 16px;
  margin-top: 20px;
}
.subscription {
  position: absolute;
  right: 20px;
  top: -24px;
  backdrop-filter: blur(20px);
  background-color: #f5f5f5f5;
  border-radius: 12px;
  box-shadow: -2px 2px 16px #94949440;
  padding: 24px 24px 20px 32px;
  width: 360px;
  transition: transform 0.25s ease;
  transform: scale(1);
  &:hover {
    transform: scale(1.03);
  }
}
.amount-wrap {
  display: flex;
  justify-content: space-between;
  padding-bottom: 20px;
}
.label {
  @mixin bold 12px;
  color: rgb(117, 117, 117);
}
.amount {
  @mixin bold 16px;
  line-height: 1.5em;
}
.subscribe-wrap {
  display: flex;
}
.subscribe-button {
  width: 133px;
  background-color: $primary;
  height: 48px;
  border-radius: 8px;
  padding: 0 16px;
  align-items: flex-start;
  margin: 20px 0 0 auto;
}
.subscribe-with {
  @mixin bold 9px;
  color: #c7c8ca;
}
.subscribe-img {
  width: 100%;
  margin-top: 3px;
}
.divider {
  width: 100%;
  height: 1px;
  background-color: white;
}
.pay-dots {
  width: 38px;
}
.right-content {
  max-width: 173px;
  margin: 0 auto;
  align-items: flex-start;
}
.gs-title {
  @mixin extra 36px;
  line-height: 1.1em;
  position: relative;
}
.gs-text {
  @mixin text 15px;
  margin-top: 12px;
  position: relative;
}
.gs-button {
  position: relative;
  margin-top: 24px;
}
@media (max-width: 1000px) {
  .why-items {
    max-width: 601px;
  }
  .why-item {
    padding: 24px 20px 8px 28px;
  }
  .v3 {
    display: block;
  }
  .v2,
  .v4 {
    display: none;
  }
  .h4 {
    display: block;
  }
  .hero-sections {
    max-width: 750px;
  }
  .subscription {
    right: 12px;
  }
  .image-wrap {
    left: 12px;
  }
  .gs-button {
    margin-top: 28px;
  }
  .hero-right {
    padding: 64px 0;
  }
}
@media (max-width: 800px) {
  .why-items {
    margin-top: 600px;
    margin-bottom: 40px;
  }
  .hero-sections {
    flex-direction: column;
    width: 90%;
    max-width: 615px;
  }
  .hero-right {
    width: 100%;
    margin: 12px 0 0 0;
  }
  .right-content {
    max-width: 400px;
    width: 100%;
    text-align: center;
    align-items: center;
    padding: 0 24px;
  }
  .gs-button {
    margin-top: 64px;
  }
  .hero-left {
    padding: 180px 0 0;
    margin: 0;
  }
  .left-content {
    text-align: center;
    padding: 0 32px 40px;
  }
}
@media (max-width: 600px) {
  .why-items {
    max-width: 300px;
  }
  .why-item {
    display: flex;
    flex-direction: column;
    text-align: center;
    align-items: center;
    padding: 32px 20px 8px 28px;
  }
  .v1,
  .v3,
  .v5 {
    display: none;
  }

  .gs-button {
    margin-top: 32px;
  }
  .right-content {
    max-width: 320px;
  }
  .left-title {
    font-size: 28px;
  }
  .left-text {
    font-size: 15px;
  }
  .subscription {
    width: 300px;
    right: 40px;
  }
  .image-wrap {
    width: 320px;
    left: 50px;
  }
}
</style>
