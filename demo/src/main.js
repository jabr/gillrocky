import Lattice from "./Lattice.svelte"

export default new Lattice({
  target: document.getElementById('lattice'),
  props: {
    total: 25_000,
    cells: 10,
    kD: 0.1
  }
})
