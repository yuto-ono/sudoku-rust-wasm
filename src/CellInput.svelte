<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte"

  export let num: number
  let oldValue = ""
  let element: HTMLInputElement
  const dispatch = createEventDispatcher<{
    input: { num: number }
    mount: { element: HTMLInputElement }
  }>()

  const onInput = ({ currentTarget }: { currentTarget: HTMLInputElement }) => {
    if (/^\d?$/.test(currentTarget.value)) {
      oldValue = currentTarget.value
      dispatch("input", { num: toNumber(currentTarget.value) })
    } else {
      currentTarget.value = oldValue
    }
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
