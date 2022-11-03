<script lang="ts" context="module">
  let timeout: NodeJS.Timeout | undefined
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte"

  export let num: number
  let oldValue = ""
  let element: HTMLInputElement
  const dispatch = createEventDispatcher<{
    input: { num: number }
    mount: { element: HTMLInputElement }
    move: { direction: "up" | "down" | "left" | "right" }
  }>()

  const onInput = ({ currentTarget }: { currentTarget: HTMLInputElement }) => {
    if (/^[1-9]?$/.test(currentTarget.value)) {
      oldValue = currentTarget.value
      dispatch("input", { num: toNumber(currentTarget.value) })
    } else {
      currentTarget.value = oldValue
    }
  }

  const onKeyDown = ({ keyCode }: { keyCode: number }) => {
    switch (keyCode) {
      case 37:
        dispatch("move", { direction: "left" })
        break
      case 38:
        dispatch("move", { direction: "up" })
        break
      case 39:
        dispatch("move", { direction: "right" })
        break
      case 40:
        dispatch("move", { direction: "down" })
        break
    }
  }

  const onFocus = () => {
    if (timeout != null) {
      clearTimeout(timeout)
    }
    timeout = setTimeout(() => element.select(), 10)
  }

  const toNumber = (value: string): number => {
    return value === "" ? 0 : Number(value)
  }

  onMount(() => {
    dispatch("mount", { element })
  })
</script>

<input
  class="input"
  type="tel"
  value={num === 0 ? "" : num}
  on:input={onInput}
  on:keydown={onKeyDown}
  on:focus={onFocus}
  bind:this={element}
/>

<style lang="scss">
  .input {
    border: none;
    width: 95%;
    height: 95%;
    background: transparent;
    text-align: center;
    font-family: inherit;
    font-size: inherit;
  }
</style>
