import { Seconds } from "rhythm.js";

type NewType<T> = T & { readonly __brand: unique symbol };

type Milliseconds = NewType<number>;

const HOUR_S = 3600 as Seconds;
const MINUTE_MS = 60000 as Milliseconds;
const HOUR_MS = (HOUR_S * 1000) as Milliseconds;

class Clock {
    static date_creation_time = Date.now() as Milliseconds;
    static performance_creation_time = performance.now() as Milliseconds;

    static high_res_now(): Date {
        return new Date(this.since_epoch());
    }

    static epoch_to_hour_start(): Milliseconds {
        const high_res_now = this.high_res_now();
        high_res_now.setMinutes(0, 0, 0);
        return high_res_now.getTime() as Milliseconds;
    }

    static epoch_to_next_hour(): Milliseconds {
        return (this.epoch_to_hour_start() + HOUR_MS) as Milliseconds;
    }

    static since_epoch(): Milliseconds {
        return (this.date_creation_time + (
            performance.now() - this.performance_creation_time
        )) as Milliseconds;
    }

    static since_hour_start(): Milliseconds {
        return (this.since_epoch() - this.epoch_to_hour_start()) as Milliseconds;
    }

    static to_next_hour(): Milliseconds {
        return (this.epoch_to_next_hour() - this.since_epoch()) as Milliseconds;
    }

    static to_next_hour_s(): Seconds {
        return this.to_s(this.to_next_hour());
    }

    static current_hour(): number {
        return this.high_res_now().getHours();
    }

    static next_hour(): number {
        return (this.current_hour() + 1) % 24;
    }

    static to_ms(seconds: Seconds): Milliseconds {
        return (seconds * 1000) as Milliseconds;
    }

    static to_s(milliseconds: Milliseconds): Seconds {
        return (milliseconds / 1000) as Seconds;
    }
}

export { type Milliseconds, Clock, HOUR_S, HOUR_MS, MINUTE_MS };