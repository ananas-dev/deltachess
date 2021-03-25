<script lang="ts">
  import { onMount } from "svelte";
  import { Chess } from "chess.ts";
  import * as d3 from "d3";
  import type { square } from "./types/types";
  export let name: string;
  let d3Ref;

  const chess = new Chess();

  onMount(() => {
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

    const div: any = d3
      .select(d3Ref)
      .append("div")
      .style("top", marginTop + "px")
      .style("left", marginLeft + "px")
      .style("width", boardSize + "px")
      .style("height", boardSize + "px");

    const svg: any = div
      .append("svg")
      .attr("width", boardSize + "px")
      .attr("height", boardSize + "px")
      .selectAll(".fields")
      .data(board)
      .enter();

    svg
      .append("rect")
      .style("class", "fields")
      .style("class", "rects")
      .attr("x", (d: square) => {
        return d.x * fieldSize;
      })
      .attr("y", (d: square) => {
        return d.y * fieldSize;
      })
      .attr("width", fieldSize + "px")
      .attr("height", fieldSize + "px")
      .style("fill", (d: square) => {
        let test = 0;
        if (d.y % 2 != 0) test = 1;
        if ((d.x + test) % 2 == 0) return "#f0d9b5";
        else return "#b58863";
      });

    svg
      .filter((d: square) => d.color != undefined)
      .append("svg:image")
      .attr("x", (d: square) => {
        return d.x * fieldSize;
      })
      .attr("y", (d: square) => {
        return d.y * fieldSize;
      })
      .attr("width", fieldSize + "px")
      .attr("height", fieldSize + "px")
      .attr(
        "xlink:href",
        (d: square) => `/assets/pieces/${d.color}/${d.type}.svg`
      )
      .classed("draggable", true);
  });
</script>

<main>
  <div bind:this={d3Ref} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
