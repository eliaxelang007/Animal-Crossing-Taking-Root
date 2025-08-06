type NewType<T> = T & { readonly __brand: unique symbol };

type Milliseconds = NewType<number>;

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
        const HOUR_MILLISECONDS = 3600000;
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
}

const clock = new Clock();

export { type Milliseconds, clock };