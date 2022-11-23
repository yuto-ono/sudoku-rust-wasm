<script lang="ts">
  export let variant: "primary" | "secondary"
  export let disabled = false
</script>

<button type="button" class="btn {variant}" on:click {disabled}>
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
    &:disabled {
      cursor: default;
      opacity: 0.5;
    }
    &.primary {
      background-color: #369;
    }
    &.secondary {
      background-color: #666;
    }
  }
</style>
