<script lang="ts">
  import Board from "./Board.svelte"
  import Title from "./Title.svelte"
</script>

<main class="main">
  <Title>数独を一瞬で解いてしまうプログラム</Title>
  <Board />
</main>

<style lang="scss">
  .main {
    padding: 30px 10px 80px;
  }
</style>
