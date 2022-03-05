// Class to calculate FPS (and ms per frame).
export class fps {
    constructor() {
        this.frames = [];
        this.last_time = -1;
        this.mean = 0.0;
    }

    // reset the time to measure from (Used when updates aren't being run).
    freeze() {
        this.last_time = -1;
    }

    // Update the counter with the current time stamp, and return the mean ms/frame.
    update() {
        // If last_time is -1, need to just record the current time and exit.
        if (this.last_time === -1) {
            this.last_time = performance.now();
        } else {
            // Otherwise, add to the framerate counter.
            const now = performance.now();
            const delta = now - this.last_time;
            this.last_time = now;

            // Calculate statistics for last 100 frames.
            this.frames.push(delta);
            if (this.frames.length > 100) {
                const x = this.frames.shift();
                // Remove from incremental average. - n=new number of elements.
                this.mean += (this.mean - x) / 99;
            }

            // Incremental average.
            this.mean += (delta - this.mean) / this.frames.length;
        }

        return this.mean;
    }

    get_fps() {
        return 1000.0 / this.mean;
    }
};

