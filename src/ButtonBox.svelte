<script lang="ts">
  import Button from "./Button.svelte"
</script>

<div class="btn-box">
  <div class="btn-wrapper">
    <Button variant="primary">実行</Button>
  </div>
  <div class="btn-wrapper">
    <Button variant="secondary">リセット</Button>
  </div>
</div>

<style lang="scss">
  .btn-box {
    display: flex;
    justify-content: center;
    margin-top: 20px;
  }
  .btn-wrapper {
    margin-left: 10px;
    margin-right: 10px;
  }
</style>
