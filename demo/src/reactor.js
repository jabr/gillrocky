// Simple 1D lattice diffusion.
class Reactor {
    constructor(cells, kD) {
        this.time = 0.0
        this.steps = 0n
        this.cells = new Uint32Array(cells)
        this.kD = kD
        this.rates = [ [0,0,0] ]
        this.rates.sum = 1
    }

    // Use Gillespie algorithm to choose when and what event happens next.
    step() {
        // Update events and rates:
        this.update()

        // Choose which event occurs:
        let [ rate, from, to ] = this.choose()
        this.cells[from]--
        this.cells[to]++

        // Determine the elapsed time:
        this.time += this.delta()
        this.steps++
    }

    // Determine the possible events and their current rate.
    update() {
        this.rates = [...this.cells].flatMap((count, index) => {
            // todo: support wrap-around boundaries
            return [
                // diffuse left
                [ this.kD * count, index, Math.max(index - 1, 0) ],
                // diffuse right
                [ this.kD * count, index, Math.min(index + 1, this.cells.length - 1) ]
            ]
        })
        this.rates.sum = this.rates.reduce((s, e) => s + e[0], 0.0)
    }

    // Select an elapsed time from the probability distribution.
    delta() {
        return -Math.log(Math.random()) / this.rates.sum
    }

    // Randomly select a specific event to occur, weighted by the relative rates.
    choose() {
        let target = Math.random() * this.rates.sum
        let index = 0
        for (; index < this.rates.length - 1; index++) {
            var rate = this.rates[index][0]
            if (target < rate) break
            target -= rate
        }
        return this.rates[index]
    }

    // Utility function to run the reactor step N times.
    multistep(n) {
        while (n-- > 0) this.step()
    }
}
