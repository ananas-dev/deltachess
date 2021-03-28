import { writable } from "svelte/store";

export const theme = writable({
  colors: {
    primary: "#f0d9b5",
    secondary: "#b58863",
  },
});

export const board = writable(undefined);
