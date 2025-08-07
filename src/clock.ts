import { Seconds } from "rhythm.js";

type NewType<T> = T & { readonly __brand: unique symbol };

type Milliseconds = NewType<number>;

const HOUR_SECONDS = 3600 as Seconds;
const MINUTE_MILLISECONDS = 60000 as Milliseconds;
const HOUR_MILLISECONDS = (HOUR_SECONDS * 1000) as Milliseconds;

class Clock {
    readonly date_creation_time: Milliseconds;
    readonly performance_creation_time: Milliseconds;

    constructor() {
        this.date_creation_time = Date.now() as Milliseconds;
        this.performance_creation_time = performance.now() as Milliseconds;
    }

    high_res_now(): Date {
        return new Date(this.since_epoch());
    }

    epoch_to_hour_start(): Milliseconds {
        const high_res_now = this.high_res_now();
        high_res_now.setMinutes(0, 0, 0);
        return high_res_now.getTime() as Milliseconds;
    }

    epoch_to_next_hour(): Milliseconds {
        return (this.epoch_to_hour_start() + HOUR_MILLISECONDS) as Milliseconds;
    }

    since_epoch(): Milliseconds {
        return (this.date_creation_time + (
            performance.now() - this.performance_creation_time
        )) as Milliseconds;
    }

    since_hour_start(): Milliseconds {
        return (this.since_epoch() - this.epoch_to_hour_start()) as Milliseconds;
    }

    to_next_hour(): Milliseconds {
        return (this.epoch_to_next_hour() - this.since_epoch()) as Milliseconds;
    }

    current_hour(): number {
        return this.high_res_now().getHours();
    }
}

const clock = new Clock();

export { type Milliseconds, clock, HOUR_SECONDS, HOUR_MILLISECONDS, MINUTE_MILLISECONDS };