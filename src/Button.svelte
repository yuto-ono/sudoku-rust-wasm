<script lang="ts">
  export let variant: "primary" | "secondary"
</script>

<button type="button" class="btn {variant}" on:click>
  <slot />
</button>

<style lang="scss">
  .btn {
    display: inline-block;
    border: none;
    outline: none;
    padding: 5px 20px;
    border-radius: 6px;
    color: #fff;
    font-size: 20px;
    font-weight: bold;
    cursor: pointer;
    user-select: none;
    &:hover {
      opacity: 0.8;
    }
    &.primary {
      background-color: #69c;
    }
    &.secondary {
      background-color: #bbb;
    }
  }
</style>
