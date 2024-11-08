<template>
  <div class="faq-wrap">
    <div class="faq-top">
      <img class="page-bg" :src="FaqBg" />
      <h1 class="faq-title"><span>Frequently Asked </span>Questions</h1>
      <Scroller :width="118" :img="FaqScroll" class="scroll1" />
    </div>
    <div class="faq-title2">Frequently Asked Questions</div>
    <div class="faq-subtitle">
      We're here to help! Browse our frequently asked questions to find answers to common
      questions and get the information you need.
    </div>
    <div class="faq-label">What is Paynetic</div>
    <div class="label-text">
      Paynetic is a cryptocurrency payment platform that enables businesses to collect
      recurring payments using smart contracts. With Paynetic, users can pay for services
      or products in a decentralized way, ensuring reliable and seamless transactions
      without traditional financial intermediaries. Our platform is a drop-in alternative
      to other recurring payment systems, such as Stripe, but with the added benefits of
      blockchain technology.
    </div>
    <div class="faq-section">
      <FaqItem :question="ts('faq.what1')" :answer="ts('faq.what1_a')" />
      <FaqItem :question="ts('faq.what2')" :answer="ts('faq.what2_a')" />
    </div>
    <div class="section-border"></div>
    <div class="faq-label">Features and Functionality</div>
    <div class="faq-section">
      <FaqItem :question="ts('faq.feat1')" :answer="ts('faq.feat1_a')" />
      <FaqItem :question="ts('faq.feat2')" :answer="ts('faq.feat2_a')" />
      <FaqItem :question="ts('faq.feat3')" :answer="ts('faq.feat3_a')" />
      <FaqItem :question="ts('faq.feat4')" :answer="ts('faq.feat4_a')" />
      <FaqItem :question="ts('faq.feat5')" :answer="ts('faq.feat5_a')" />
      <FaqItem :question="ts('faq.feat6')" :answer="ts('faq.feat6_a')" />
    </div>
    <div class="section-border"></div>
    <div class="faq-label">How Paynetic Works</div>
    <div class="faq-section">
      <FaqItem :question="ts('faq.how1')" :answer="ts('faq.how1_a')" />
      <FaqItem :question="ts('faq.how2')" :answer="ts('faq.how2_a')" />
      <FaqItem :question="ts('faq.how3')" :answer="ts('faq.how3_a')" />
      <FaqItem :question="ts('faq.how4')" :answer="ts('faq.how4_a')" />
      <FaqItem :question="ts('faq.how5')" :answer="ts('faq.how5_a')" />
    </div>
    <div class="section-border"></div>
    <div class="label-text">
      If you have more questions, please contact us! We’re here to make
      cryptocurrency-based recurring payments easy, reliable, and efficient.
    </div>
    <div class="faq-bottom">
      <PayneticInfo />
      <HomeScale class="ready" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import FaqItem from '../components/home/FaqItem.vue'
import HomeScale from '../components/home/HomeScale.vue'
import FaqBg from '../assets/img/faq-bg.jpg'
import FaqScroll from '../assets/img/faq-scroll.svg'
import { ts } from '../i18n'
import Scroller from '../components/widgets/Scroller.vue'
import PayneticInfo from '../components/home/PayneticInfo.vue'
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.faq-wrap {
  position: relative;
  text-align: center;
  background-color: white;
}
.page-bg {
  height: 480px;
}
.faq-top {
  position: relative;
  height: 480px;
}
.faq-title {
  @mixin extra 54px;
  padding-top: 210px;
  margin-top: 0;
  position: relative;
  color: white;
  span {
    font-weight: 300;
  }
}
.scroll1 {
  bottom: 0;
  opacity: 0.4;
}
.faq-title2 {
  @mixin bold 36px;
  color: $text2;
  margin-top: 110px;
}

.faq-subtitle {
  @mixin text 24px;
  max-width: 793px;
  margin: 24px auto 40px;
  text-align: center;
  position: relative;
}
.faq-label {
  @mixin title 24px;
  margin: 48px auto 0;
  color: $text2;
  text-align: center;
  position: relative;
}
.label-text {
  @mixin text 16px;
  color: $text1;
  max-width: 864px;
  margin: 10px auto 32px;
  text-align: center;
  position: relative;
}
.faq-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  max-width: 80%;
  position: relative;
  margin: 0 auto;
}
.section-border {
  width: 75%;
  max-width: 900px;
  margin: 56px auto 64px;
  height: 1px;
  background-color: $highlight;
  position: relative;
}
.faq-scale {
  margin-top: 90px;
}
.faq-bottom {
  display: flex;
  height: 600px;
  position: relative;
}
.ready {
  width: 50%;
}
@media (max-width: 1000px) {
  .faq-title {
    max-width: 500px;
    font-size: 48px;
    margin: 0 auto;
  }
  .faq-subtitle {
    max-width: 75%;
    width: 100%;
  }
  .faq-label {
    max-width: 75%;
    width: 100%;
  }
  .label-text {
    max-width: 75%;
    width: 100%;
  }
}
@media (max-width: 860px) {
  .faq-bottom {
    flex-direction: column;
    height: auto;
  }
  .ready {
    width: 100%;
  }
}
@media (max-width: 600px) {
  .faq-subtitle {
    max-width: 90%;
  }
  .faq-label {
    max-width: 90%;
  }
  .label-text {
    max-width: 90%;
  }
}
</style>
