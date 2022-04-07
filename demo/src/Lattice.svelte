<script>
  export let total
  export let cells
  export let kD

  import Reactor from "./reactor.js"
  const reactor = window.reactor = new Reactor(cells, kD)
  reactor.cells[0] = total
  $: t = reactor.time.toExponential(3)
  $: n = reactor.steps / 1000n

  let interval = setInterval(() => {
    reactor.multistep(1000)
    reactor = reactor // trigger svelte update - @todo: better way to do this?
    if (reactor.time > 1000) clearInterval(interval)
  }, 10)

</script>

<table>
  <tr>
    {#each reactor.cells as count}
      <td><div style="height: {count * 100 / total}%;"/></td>
    {/each}
  </tr>
  <caption>t={t} n={n}k</caption>
</table>

<style>
  table {
    height: 100%;
    width: 100%;
    background: #aaa;
    border-spacing: 2px;
  }
  table caption {
    caption-side: bottom;
    text-align: right;
  }
  table tr td {
    background: #ccc;
    position: relative;
  }
  table tr td div {
    background: #aaf;
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 0%;
  }
</style>
