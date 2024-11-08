<template>
  <div class="integration-wrap">
    <div class="integration-top">
      <img class="page-bg" :src="IntegrationBg" />
      <h1 class="integration-title">Integrations</h1>
      <Scroller :width="475" :img="IntegrationScroll" class="scroll1" />
    </div>
    <div class="section1">
      <div class="section1-bg-wrap overlay">
        <img class="section1-bg" :src="Connections" />
      </div>
      <div class="section1-content">
        <div class="section1-label">
          Unlock new possibilities with Paynetic's integrations. Enhance your platform and
          offer more to your customers.
        </div>
        <div class="text1">
          Paynetic is designed to simplify your life, not complicate it. That's why we've
          made it incredibly easy to integrate with the platforms and tools you already
          use every day. Whether it's connecting to your preferred e-commerce platform,
          integrating with major cryptocurrency wallets like MetaMask and WalletConnect,
          or linking up with business automation tools, Paynetic seamlessly fits into your
          existing workflows.
        </div>
        <div class="text1">
          Our user-friendly integration process ensures you can connect and configure your
          systems effortlessly. Spend less time on setup and more time focusing on what
          matters most: growing your business and providing exceptional service to your
          customers. With Paynetic's integrations, you can streamline your operations,
          enhance efficiency, and unlock new levels of convenience in managing your
          cryptocurrency subscriptions.
        </div>
      </div>
      <div class="integration-h"></div>
    </div>
    <div class="section2">
      <div class="section2-title">Integrations for Every Platform</div>
      <div class="text2">
        Paynetic seamlessly connects with the tools you already use, including:
      </div>
      <IntegrationTypes :light="true" />
    </div>
    <div class="integration-bottom">
      <PayneticInfo />
      <HomeScale class="ready" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import Connections from '../assets/img/connections.jpg'
import IntegrationBg from '../assets/img/integration-bg.jpg'
import IntegrationScroll from '../assets/img/integration-scroll.svg'
import HomeScale from '../components/home/HomeScale.vue'
import IntegrationTypes from '../components/home/IntegrationTypes.vue'
import PayneticInfo from '../components/home/PayneticInfo.vue'
import Scroller from '../components/widgets/Scroller.vue'
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.integration-wrap {
  position: relative;
  text-align: center;
  background-color: white;
}
.page-bg {
  height: 480px;
}
.integration-top {
  position: relative;
  height: 480px;
}
.integration-title {
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
.section1-bg-wrap {
  height: 600px;
  mask: linear-gradient(0deg, rgba(0, 0, 0, 0) 0%, rgb(0, 0, 0) 100%);
  opacity: 0.2;
}
.section1-bg {
  object-fit: cover;
  width: 100%;
  height: 100%;
}
.scroll1 {
  bottom: 0;
  opacity: 0.5;
}
.section1-content {
  max-width: 940px;
  margin: 0 auto;
}
.section1-label {
  @mixin bold 24px;
  color: $text2;
}
.text1 {
  @mixin text 16px;
  margin-top: 24px;
}
.integration-h {
  width: 100%;
  max-width: 960px;
  height: 1px;
  background-color: rgba(181, 216, 205, 0.25);
  margin: 64px auto;
}
.section2 {
  padding: 16px 0 180px;
}
.section2-title {
  @mixin bold 36px;
  color: $text2;
  padding: 0 64px;
}
.text2 {
  @mixin text 16px;
  margin: 20px 0 40px;
  padding: 0 64px;
}
:deep(.integration-types) {
  color: $text2;
  .type {
    background-color: rgba(182, 217, 206, 0.1);
  }
  .type-label {
    border-color: #d8e2e1;
  }
}
.integration-bottom {
  display: flex;
  height: 600px;
  position: relative;
}
.ready {
  width: 50%;
}

@media (max-width: 860px) {
  .integration-bottom {
    flex-direction: column;
    height: auto;
  }
  .ready {
    width: 100%;
  }
  .section2 {
    padding-bottom: 120px;
  }
}

@media (max-width: 600px) {
  .section1 {
    padding: 96px 32px 0;
  }
}
</style>
