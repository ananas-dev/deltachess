<script lang="ts">
  import { onMount } from "svelte";
  import { Chess } from "chess.ts";
  import * as d3 from "d3";
  import type { square } from "../types/types";

  let ref;

  const chess = new Chess();

  const marginTop = 30,
    marginLeft = 30,
    fieldSize = 80,
    boardDimension = 8,
    boardSize = boardDimension * fieldSize;

  let board: any = [];
  const ranks = "abcdefgh".split("");

  chess.board().map((row: any[], i: number) => {
    row.map((square, j) => {
      board.push({
        x: (i * boardDimension + j) % boardDimension,
        y: Math.floor((i * boardDimension + j) / boardDimension),
        rank: ranks[(i * boardDimension + j) % boardDimension],
        file: Math.floor((i * boardDimension + j) / boardDimension),
        color: square?.color,
        type: square?.type,
      });
    });
  });
</script>

{#each board as squares}
  <div />
{/each}
