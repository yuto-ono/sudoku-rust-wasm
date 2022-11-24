<script lang="ts">
  import CellInput from "./CellInput.svelte"
  import { cells } from "./stores"

  const elements: HTMLInputElement[] = []

  const onInput = (num: number, i: number) => {
    cells.updateCell(i, { num, inputed: num !== 0 })
    elements[i + 1]?.focus()
  }

  const setElement = (element: HTMLInputElement, i: number) => {
    elements[i] = element
  }

  const onMove = (direction: "up" | "down" | "left" | "right", i: number) => {
    switch (direction) {
      case "up":
        elements[i - 9]?.focus()
        break
      case "down":
        elements[i + 9]?.focus()
        break
      case "left":
        elements[i - 1]?.focus()
        break
      case "right":
        elements[i + 1]?.focus()
        break
    }
  }
</script>

<ul class="board">
  {#each $cells as { num, inputed }, i}
    <li class="cell" class:inputed>
      <CellInput
        {num}
        on:input={(e) => onInput(e.detail.num, i)}
        on:mount={(e) => setElement(e.detail.element, i)}
        on:move={(e) => onMove(e.detail.direction, i)}
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
