<script lang="ts">
  export let size: number;
  export let primary: string;
  export let secondary: string;

  const square = {
    size: size / 8,
  };

  function getColor(x: number, y: number) {
    return (x + (y % 2 != 0 ? 1 : 0)) % 2 == 0;
  }

  function handleClick(e) {
    console.log(e.layerX);
    console.log(e.layerY);
    console.log(e);
  }
</script>

<!--basic board-->
<svg width={size} height={size} on:click={handleClick}>
  {#each { length: 8 } as _, x}
    {#each { length: 8 } as _, y}
      <rect
        width={square.size}
        height={square.size}
        fill={getColor(x, y) ? primary : secondary}
        x={x * square.size}
        y={y * square.size}
      />
    {/each}
  {/each}
</svg>
