<script lang="ts">
  import Button from "./Button.svelte"
  import { cells, cellsIsEmpty, solved, time } from "./stores"
  import { Solver } from "./solver"

  const solve = () => {
    const solver = new Solver($cells.map((cell) => cell.num))
    if (!solver.isValid) {
      alert("重複があります。")
    } else {
      const startTime = performance.now()
      $solved = solver.solve()
      $time = performance.now() - startTime
      if ($solved) {
        cells.setSolvedArray(solver.getNumberArray())
      } else {
        alert("解けませんでした。")
      }
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
      <Button variant="primary" on:click={solve} disabled={$cellsIsEmpty}>
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
