<script lang="ts">
  import Button from "./Button.svelte"
  import { cells, cellsIsEmpty, solved, time } from "./stores"
  import { solve, SolveStatus } from "./solver"

  const invokeSolve = () => {
    const startTime = performance.now()
    const numArray = Uint32Array.from($cells.map((cell) => cell.num))
    const result = solve(numArray)
    $time = performance.now() - startTime
    switch (result) {
      case SolveStatus.success:
        cells.setSolvedArray(numArray)
        $solved = true
        break
      case SolveStatus.duplicated:
        alert("重複があります。")
        break
      case SolveStatus.noEmpty:
        alert("空白のマスがありません。")
        break
      case SolveStatus.unsolvable:
        alert("解けませんでした。")
        break
      default:
        alert("不正な入力です。")
        break
    }
  }

  const reset = () => {
    cells.reset()
    $solved = false
  }

  const undo = () => {
    cells.undo()
    $solved = false
  }
</script>

<div class="btn-box">
  <div class="btn-wrapper">
    {#if !$solved}
      <Button variant="primary" on:click={invokeSolve} disabled={$cellsIsEmpty}>
        実行
      </Button>
    {:else}
      <Button variant="primary" on:click={undo}>戻す</Button>
    {/if}
  </div>
  <div class="btn-wrapper">
    <Button variant="secondary" on:click={reset} disabled={$cellsIsEmpty}>
      リセット
    </Button>
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
