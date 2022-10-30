<ul class="board">
  {#each Array(81).fill(8) as num, i}
    <li class="cell">{num}</li>
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
    font-weight: bold;
    cursor: pointer;
    user-select: none;
    @media (max-width: 620px) {
      height: calc((100vw - 20px) / 9);
      font-size: 20px;
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
