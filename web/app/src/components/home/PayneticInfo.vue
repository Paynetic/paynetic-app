<template>
  <div class="found f-center-col">
    <img src="../../assets/img/found-bg.jpg" class="overlay" />
    <div class="found-content f-center-col">
      <div class="found-title">Can't Find What You're Looking For?</div>
      <div class="found-text">
        Get in touch with us to get more information or help. We'll be happy to assist
        you.
      </div>
      <a href="mailto:info@paynetic.net" target="_blank" class="found-button">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">
          <g fill="white">
            <path d="M224,56l-96,88L32,56Z"></path>
            <path
              d="M224,48H32a8,8,0,0,0-8,8V192a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A8,8,0,0,0,224,48Zm-96,85.15L52.57,64H203.43ZM98.71,128,40,181.81V74.19Zm11.84,10.85,12,11.05a8,8,0,0,0,10.82,0l12-11.05,58,53.15H52.57ZM157.29,128,216,74.18V181.82Z"
            ></path>
          </g>
        </svg>
        info@paynetic.net
      </a>
    </div>
  </div>
</template>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.found {
  position: relative;
  width: 50%;
  background-color: black;
}
.found-content {
  max-width: 440px;
  padding: 0 40px;
  position: relative;
}
.found-title {
  @mixin bold 36px;
  color: rgb(181, 216, 205);
}
.found-text {
  @mixin text 18px;
  margin-top: 12px;
  color: white;
}
.found-button {
  @mixin text 16px;
  display: flex;
  align-items: center;
  color: white;
  padding: 6px 12px;
  border: 1px solid white;
  margin-top: 32px;
  background-color: rgb(0, 0, 0);
  transition: all 0.25s ease;
  svg {
    width: 18px;
    height: 18px;
    margin: 0 8px 0 0;
  }
  &:hover {
    transform: scale(1.1);
    background-color: rgba(255, 255, 255, 0.1);
  }
}
@media (max-width: 860px) {
  .found-content {
    padding: 120px 24px;
  }
  .found {
    width: 100%;
  }
}
</style>
