<script lang="ts">
  //import ChessboardD3 from "./components/ChessboardD3.svelte";
  import Board from "./components/Board.svelte";
  import { theme } from "./stores";
</script>

<main>
  <Board
    size={640}
    primary={$theme.colors.primary}
    secondary={$theme.colors.secondary}
  />
</main>

<style lang="scss">
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
