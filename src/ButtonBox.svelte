<div class="btn-box">
  <div class="btn-wrapper">
    <button type="button" class="btn solve">実行</button>
  </div>
  <div class="btn-wrapper">
    <button type="button" class="btn reset">リセット</button>
  </div>
</div>

<style lang="scss">
  @use "btn-base" as *;
  .btn-box {
    display: flex;
    justify-content: center;
    margin-top: 20px;
  }
  .btn-wrapper {
    &:nth-child(n + 2) {
      margin-left: 20px;
    }
  }
  .btn {
    @extend %btn-base;
  }
</style>
