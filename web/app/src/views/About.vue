<template>
  <div class="about-wrap">
    <div class="about-top">
      <img class="page-bg" :src="AboutBg" />
      <h1 class="about-title"><span>About </span>Paynetic</h1>
      <Scroller :width="331" :img="AboutScroll1" class="scroll1" />
    </div>
    <div class="section1">
      <img class="section1-bg overlay" :src="About1" />
      <div class="section1-content">
        <div
          class="section1-left label-wrap"
          :class="{ active: active === 0 }"
          @click="active = 0"
        >
          <div class="section1-title">
            Redefining Subscription Payments with Cryptocurrency and AI
          </div>
          <div class="title-divider"></div>
          <div class="left-content">
            <div class="text1">
              <span
                >At Paynetic, we believe in the future of decentralized finance.
              </span>
              We provide businesses with a secure, automated solution for cryptocurrency
              subscription payments, powered by the latest in blockchain and artificial
              intelligence technologies.
            </div>
          </div>
        </div>
        <div class="section1-right">
          <div class="label-wrap" :class="{ active: active === 1 }" @click="active = 1">
            <div class="section1-label">Our Mission</div>
            <div class="section1-text">
              To revolutionize subscription payments by delivering secure, transparent,
              and automated solutions for businesses of all sizes through AI and
              blockchain.
            </div>
          </div>
          <div class="section1-h"></div>
          <div class="label-wrap" :class="{ active: active === 2 }" @click="active = 2">
            <div class="section1-label">Our Vision</div>
            <div class="section1-text">
              We envision a world where businesses are free from the constraints of
              traditional payment systems and are able to thrive using decentralized
              technologies.
            </div>
          </div>
        </div>
      </div>
      <div class="about-select">
        <div class="select" :class="{ active: active === 0 }" @click="active = 0">
          ABOUT
        </div>
        <div class="select" :class="{ active: active === 1 }" @click="active = 1">
          MISSION
        </div>
        <div class="select" :class="{ active: active === 2 }" @click="active = 2">
          VISION
        </div>
      </div>
    </div>
    <div class="section2">
      <img class="section2-bg overlay" :src="About2" />
      <Scroller :width="407" :img="AboutScroll2" :reverse="true" class="scroll2" />
      <div class="section2-content">
        <div class="section2-title">
          <div>Our Values</div>
          <div class="title-anim" :class="{ moved: animMove }">
            {{ animText }}
          </div>
        </div>
        <div class="boxes">
          <div class="box f-center-col">
            <div class="box-overlay1 overlay"></div>
            <div class="box-overlay2 overlay"></div>
            <img src="../assets/img/innovation.svg" class="box-img" />
            <div class="box-label">Innovation</div>
            <div class="box-text">
              We continuously explore new ways to make cryptocurrency payments faster,
              safer, and more efficient.
            </div>
          </div>
          <div class="box f-center-col">
            <div class="box-overlay1 overlay"></div>
            <div class="box-overlay2 overlay"></div>
            <img src="../assets/img/security.svg" class="box-img" />
            <div class="box-label">Security</div>
            <div class="box-text">
              With AI-driven fraud detection and smart contract automation, we guarantee
              secure and transparent payment solutions.
            </div>
          </div>
          <div class="box f-center-col">
            <div class="box-overlay1 overlay"></div>
            <div class="box-overlay2 overlay"></div>
            <img src="../assets/img/scalability.svg" class="box-img" />
            <div class="box-label">Scalability</div>
            <div class="box-text">
              Whether you're a startup or an enterprise, Paynetic scales effortlessly to
              fit your growing needs.
            </div>
          </div>
          <div class="box f-center-col">
            <div class="box-overlay1 overlay"></div>
            <div class="box-overlay2 overlay"></div>
            <img src="../assets/img/transparency.svg" class="box-img" />
            <div class="box-label">Transparency</div>
            <div class="box-text">
              Blockchain ensures that every transaction is traceable and secure, offering
              unparalleled trust to businesses and their customers.
            </div>
          </div>
        </div>
      </div>
    </div>
    <HomeScale />
  </div>
</template>

<script lang="ts" setup>
import { computed, nextTick, onMounted, onUnmounted, ref } from 'vue'
import HomeScale from '../components/home/HomeScale.vue'
import AboutBg from '../assets/img/about-bg.jpg'
import About1 from '../assets/img/about1.jpg'
import About2 from '../assets/img/about2.jpg'
import Scroller from '../components/widgets/Scroller.vue'
import AboutScroll1 from '../assets/img/about-scroll1.svg'
import AboutScroll2 from '../assets/img/about-scroll2.svg'

const active = ref(0)
let animInterval: ReturnType<typeof setInterval> | undefined
const animIndex = ref(0)
const animMove = ref(false)
const animText = computed(() => {
  return ['INNOVATION', 'SECURITY', 'SCALABILITY', 'TRANSPARENCY'][animIndex.value]
})

onMounted(() => {
  if (!animInterval) {
    animInterval = setInterval(async () => {
      animMove.value = true
      await nextTick()
      animIndex.value = (animIndex.value + 1) % 4
      setTimeout(() => (animMove.value = false), 1)
    }, 1500)
  }
})

onUnmounted(() => {
  if (animInterval) {
    clearInterval(animInterval)
    animInterval = undefined
  }
})
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.about-wrap {
  position: relative;
  text-align: center;
  background-color: white;
}
.page-bg {
  height: 480px;
}
.about-top {
  position: relative;
  height: 480px;
}
.about-title {
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
}
.section1 {
  position: relative;
  height: 480px;
  margin-top: 120px;
}
.section1-content {
  display: flex;
  justify-content: space-between;
  padding: 0 60px 50px;
  position: relative;
  background-color: white;
  max-width: 950px;
  margin: 0 auto;
  text-align: left;
}
.section1-left {
  width: 55%;
  border-right: 1px solid rgba(182, 217, 206, 0.35);
  padding-right: 5%;
}
.section1-title {
  @mixin bold 32px;
  color: $text2;
  transition: color 0.35s ease;
}
.left-content {
  opacity: 0.5;
  transition: opacity 0.35s ease;
}
.title-divider {
  width: 186px;
  height: 1px;
  background-color: rgb(181, 216, 205);
  margin: 16px 0 16px;
}
.text1 {
  @mixin text 16px;
  span {
    font-weight: 700;
  }
}
.section1-right {
  width: 40%;
}
.section1-label {
  @mixin bold 16px;
  color: black;
  transition: color 0.35s ease;
}
.section1-h {
  width: 100%;
  background-color: rgba(182, 217, 206, 0.35);
  height: 1px;
  margin: 16px 0;
}
.section1-text {
  @mixin text 16px;
  margin-top: 8px;
  opacity: 0.5;
  transition: opacity 0.35s ease;
}
.label-wrap.active {
  .section1-label {
    color: $primary;
  }
  .section1-text {
    opacity: 1;
  }
  .section1-title {
    color: $text2;
  }
  .left-content {
    opacity: 1;
  }
}
.about-select {
  max-width: 820px;
  display: flex;
  justify-content: space-between;
  margin: 40px auto 0;
  position: relative;
}
.select {
  @mixin extra 36px;
  color: white;
  opacity: 0.3;
  cursor: pointer;
  transition: opacity 0.35s ease;
  &.active {
    opacity: 1;
  }
}
.scroll2 {
  top: 0;
  opacity: 0.35;
}
.section2 {
  position: relative;
  padding: 80px 40px 180px;
}
.section2-content {
  position: relative;
  max-width: 1096px;
  width: 100%;
  margin: 0 auto;
}
.section2-title {
  @mixin bold 54px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: white;
  text-align: left;
  padding-left: 40px;
}
.title-anim {
  @mixin thin 24px;
  letter-spacing: 0.5em;
  opacity: 0.5;
  color: white;
  margin-top: 12px;
  transform: translateX(0);
  transition: transform 0.2s ease;
  &.moved {
    transition: unset;
    transform: translateX(-20px);
  }
}
.boxes {
  display: flex;
  position: relative;
  justify-content: space-between;
  flex-wrap: wrap;
  margin-top: 24px;
}
.box-overlay1 {
  transition: opacity 0.35s ease;
  opacity: 1;
  background: linear-gradient(
    60deg,
    rgba(255, 255, 255, 0.8) 0%,
    rgba(255, 255, 255, 0.6) 100%
  );
}
.box-overlay2 {
  transition: opacity 0.35s ease;
  opacity: 0;
  background: linear-gradient(
    60deg,
    rgba(255, 255, 255, 0.8) 50%,
    rgba(255, 255, 255, 0.6) 100%
  );
}
.box-img {
  position: absolute;
  width: 29px;
  height: 29px;
  right: 28px;
  top: 22px;
  opacity: 0.5;
  fill: rgb(34, 79, 75);
  transition: opacity 0.35s ease;
}
.box {
  position: relative;
  width: calc(50% - 8px);
  text-align: left;
  margin-top: 12px;
  backdrop-filter: blur(20px);
  height: 202px;
  user-select: none;
  border: 2px solid rgba(255, 255, 255, 0.25);
  transition: border-color 0.35s ease;
  &:hover {
    border-color: rgb(255, 255, 255);
    .box-overlay1 {
      opacity: 0.7;
    }
    .box-overlay2 {
      opacity: 0.7;
    }
    .box-img {
      opacity: 1;
    }
  }
}
.box-label {
  @mixin bold 16px;
  color: $text2;
  max-width: 373px;
  width: 100%;
  margin: 0 auto;
  position: relative;
}
.box-text {
  @mixin text 16px;
  max-width: 373px;
  width: 100%;
  margin: 12px auto 0;
  position: relative;
}

@media (max-width: 1000px) {
  .box {
    padding-left: 40px;
    padding-right: 32px;
  }
  .section1 {
    height: auto;
  }
  .section2 {
    padding-bottom: 104px;
  }
  .section2-title {
    padding-left: 24px;
    font-size: 40px;
  }
  .title-anim {
    font-size: 20px;
  }
  .about-select {
    max-width: 80%;
    padding-bottom: 48px;
  }
}

@media (max-width: 600px) {
  .about-title {
    width: 90%;
    margin-left: auto;
    margin-right: auto;
  }
  .section1-content {
    flex-direction: column;
    text-align: center;
    padding-left: 24px;
    padding-right: 24px;
    max-width: 500px;
  }
  .title-anim {
    display: none;
  }
  .section1-left {
    width: 100%;
    border-right: none;
  }
  .title-divider {
    margin-left: auto;
    margin-right: auto;
    width: 60%;
  }
  .section1-right {
    width: 100%;
    padding-top: 24px;
    margin-top: 24px;
    border-top: 1px solid rgba(182, 217, 206, 0.35);
  }
  .about-select {
    max-width: 90%;
  }
  .select {
    font-size: 24px;
  }
  .section1-text {
    max-width: 400px;
    margin-left: auto;
    margin-right: auto;
  }
  .section2 {
    padding-top: 64px;
  }
  .section2-title {
    padding: 0;
    justify-content: center;
  }
  .boxes {
    flex-direction: column;
    align-items: center;
  }
  .box {
    width: 90%;
    max-width: 360px;
    height: auto;
    padding-top: 48px;
    padding-bottom: 48px;
  }
}
</style>
