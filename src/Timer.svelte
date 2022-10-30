<script lang="ts">
  export let time: number
</script>

<p class="timer">
  実行時間:
  <span class="time">{time}</span>ms
</p>

<style lang="scss">
  .timer {
    margin-top: 20px;
    font-size: 20px;
    text-align: center;
  }
  .time {
    display: inline-block;
    min-width: 80px;
    text-align: right;
  }
</style>
