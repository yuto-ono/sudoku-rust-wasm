<script lang="ts">
  export let variant: "solve" | "reset"
</script>

<button type="button" class="btn {variant}">
  <slot />
</button>

<style lang="scss">
  @use "btn-base" as *;
  .btn {
    @extend %btn-base;
    padding: 5px 20px;
    border-radius: 6px;
    background-color: #69c;
    color: #fff;
    font-size: 20px;
    font-weight: bold;
    &.solve {
      background-color: #69c;
    }
    &.reset {
      background-color: #bbb;
    }
  }
</style>
