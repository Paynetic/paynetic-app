<template>
  <div class="how-connect f-center">
    <div class="img-wrap">
      <img src="../../assets/img/connect3.svg" class="how3" />
      <img src="../../assets/img/connect1.svg" class="how1" />
      <img src="../../assets/img/connect2.svg" class="how2" />
    </div>
  </div>
</template>

<style lang="postcss" scoped>
@import '../../css/defines.postcss';

.how-connect {
  width: 100%;
  height: 100%;
}
.img-wrap {
  position: relative;
}
.how3 {
  width: 202px;
  height: 202px;
}
.how1 {
  position: absolute;
  left: -48px;
  top: -4px;
  width: 115px;
  height: 115px;
}
.how2 {
  position: absolute;
  width: 178px;
  height: 178px;
  right: -100px;
  bottom: -24px;
}
</style>
