<template>
  <div class="features-wrap">
    <div class="features-top">
      <img class="page-bg" :src="FeaturesBg" />
      <h1 class="features-title">Features</h1>
      <Scroller :width="328" :img="FeaturesScroll" class="scroll1" />
    </div>
    <div class="features-content">
      <div class="content-scrollers">
        <Scroller :width="355" :reverse="true" :img="FeaturesScroll2" class="scroll2" />
        <Scroller :width="416" :img="FeaturesScroll3" class="scroll3" />
        <Scroller :width="254" :reverse="true" :img="FeaturesScroll4" class="scroll4" />
      </div>
      <div class="features-label">Powerful, Automated, and Secure Crypto Payments</div>
      <div class="boxes">
        <FeatureBox
          v-for="feature in features"
          :title="feature.title"
          :text="feature.text"
          :reverse="feature.reverse"
          :img="feature.img"
        />
      </div>
    </div>
    <HomeScale />
  </div>
</template>

<script lang="ts" setup>
import FeaturesBg from '../assets/img/features-bg.jpg'
import FeaturesScroll from '../assets/img/feature-page-scroll.svg'
import FeaturesScroll2 from '../assets/img/features-scroll2.svg'
import FeaturesScroll3 from '../assets/img/features-scroll3.svg'
import FeaturesScroll4 from '../assets/img/features-scroll4.svg'
import Ai from '../assets/img/ai.gif'
import Blockchain from '../assets/img/blockchain.gif'
import Crypto from '../assets/img/crypto.gif'
import Fraud from '../assets/img/fraud.gif'
import Analytics from '../assets/img/analytics.gif'
import HomeScale from '../components/home/HomeScale.vue'
import Scroller from '../components/widgets/Scroller.vue'
import FeatureBox from '../components/home/FeatureBox.vue'

const features = [
  {
    title: 'AI-Powered Payment Automation',
    text: 'Paynetic’s AI automatically optimizes subscription billing, predicting and preventing failed payments, and enhancing user engagement through behavioral insights.',
    img: Ai,
  },
  {
    title: 'Blockchain Smart Contracts',
    text: 'Automate your recurring payments with blockchain technology. Paynetic ensures every transaction is transparent, reliable, and secure without the need for manual intervention.',
    img: Blockchain,
    reverse: true,
  },
  {
    title: 'Support for Multiple Cryptocurrencies',
    text: 'Accept Ethereum, ERC-20 tokens, and more, giving your global audience the flexibility they need. Integration with major wallets ensures smooth and efficient payment processing.',
    img: Crypto,
  },
  {
    title: 'Advanced Fraud Detection',
    text: 'AI continuously monitors your transactions, helping you prevent fraud before it happens, ensuring the safety of your funds and customer data.',
    img: Fraud,
    reverse: true,
  },
  {
    title: 'Real-Time Reporting & Analytics',
    text: 'Paynetic provides detailed analytics on subscription revenue and user behavior. AI-powered insights allow you to optimize strategies and reduce churn.',
    img: Analytics,
  },
]
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.features-wrap {
  position: relative;
  text-align: center;
  background-color: white;
}
.page-bg {
  height: 480px;
}
.features-top {
  position: relative;
  height: 480px;
}
.features-title {
  @mixin extra 54px;
  padding-top: 210px;
  margin-top: 0;
  position: relative;
  color: white;
}
.section1 {
  position: relative;
  padding: 180px 64px 0;
}
.content-scrollers {
  position: absolute;
  top: 170px;
  left: 0;
  width: 100%;
}
.scroll1 {
  bottom: 0;
  opacity: 0.6;
}
.scroll2 {
  position: relative;
  opacity: 0.7;
  left: -280px;
}
.scroll3 {
  position: relative;
  opacity: 0.7;
  margin-top: 18px;
  left: -36px;
}
.scroll4 {
  position: relative;
  top: 0;
  opacity: 0.7;
  margin-top: 18px;
  left: -160px;
}
.features-content {
  max-width: 1080px;
  padding: 170px 40px 140px;
  position: relative;
  margin: 0 auto;
}
.features-label {
  @mixin extra 54px;
  padding-top: 16px;
  color: $text2;
  position: relative;
  max-width: 820px;
  margin: 0 auto;
}
.boxes {
  margin-top: 124px;
}

@media (max-width: 600px) {
}
</style>
