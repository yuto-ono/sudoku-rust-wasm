<script lang="ts">
  import CellInput from "./CellInput.svelte"
  import { CELL_NUMBER } from "./constants"
  import { cells } from "./stores"

  const onInput = (num: number, i: number) => {
    cells.updateCell(i, { num, inputed: num !== 0 })
    if (i + 1 < CELL_NUMBER) {
      $cells[i + 1].element?.focus()
    }
  }

  const setElement = (element: HTMLInputElement, i: number) => {
    cells.updateCell(i, { element })
  }
</script>

<ul class="board">
  {#each $cells as { num, inputed }, i}
    <li class="cell" class:inputed>
      <CellInput
        {num}
        on:input={(e) => onInput(e.detail.num, i)}
        on:mount={(e) => setElement(e.detail.element, i)}
      />
    </li>
  {/each}
</ul>

<style lang="scss">
  .board {
    display: flex;
    flex-wrap: wrap;
    list-style: none;
  }
  .cell {
    display: flex;
    align-items: center;
    justify-content: center;
    width: calc(100% / 9);
    height: 55px;
    border-top: 1px solid #ccc;
    border-left: 1px solid #ccc;
    font-size: 24px;
    user-select: none;
    @media (max-width: 620px) {
      height: calc((100vw - 20px) / 9);
      font-size: 20px;
    }
    &.inputed {
      background-color: #ff9;
    }
    &:nth-child(9n) {
      border-right: 2px solid #999;
    }
    &:nth-child(3n + 1) {
      border-left: 2px solid #999;
    }
    &:nth-child(n + 73) {
      border-bottom: 2px solid #999;
    }
    &:nth-child(-n + 9),
    &:nth-child(n + 28):nth-child(-n + 36),
    &:nth-child(n + 55):nth-child(-n + 63) {
      border-top: 2px solid #999;
    }
  }
</style>
