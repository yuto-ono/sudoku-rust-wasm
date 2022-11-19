<script lang="ts">
  import Board from "./Board.svelte"
  import ButtonBox from "./ButtonBox.svelte"
  import SampleButtonBox from "./SampleButtonBox.svelte"
  import Timer from "./Timer.svelte"
  import Title from "./Title.svelte"
</script>

<main class="main">
  <Title>数独を一瞬で解いてしまうプログラム（Rust版）</Title>
  <div class="container">
    <Board />
    <ButtonBox />
    <Timer />
    <SampleButtonBox />
  </div>
</main>

<style lang="scss">
  .main {
    padding: 30px 10px 80px;
  }
  .container {
    max-width: 500px;
    margin-left: auto;
    margin-right: auto;
  }
</style>
