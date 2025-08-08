import { Play, Sequence, Repeat, RhythmContext, Seconds, Gain } from "rhythm.js";
import { Clock, HOUR_S, Milliseconds, MINUTE_MS } from "./clock";
import { gsap } from "gsap";
import { Store } from "./store";

const delay = (wait: Milliseconds) => new Promise((resolve) => setTimeout(resolve, wait));

const context = new AudioContext();
const rhythm = new RhythmContext(context);

const hour_bells = await rhythm.compile(new Play("audios/hour_bells.oga"));

const fade_in = context.createGain();
fade_in.connect(context.destination);

const FADE_OUT_DURATION = 20 as Seconds;

const songs_store = new Store(
    async (hour: number) => {
        const song = await rhythm.compile(
            new Gain(
                {
                    gain_keyframes: [
                        {
                            transition: undefined,
                            value: 1,
                            from_start: (HOUR_S - FADE_OUT_DURATION) as Seconds
                        },
                        {
                            transition: "exponential",
                            value: 0.01, // Can't exponentially fade out to a flat 0.
                            from_start: HOUR_S
                        }
                    ]
                },
                new Sequence(
                    [
                        hour_bells,
                        new Repeat(
                            {
                                duration: (HOUR_S - hour_bells.duration) as Seconds
                            },
                            new Play(`audios/${hour}.oga`)
                        )
                    ]
                )
            )
        );

        return song.attach_to(fade_in);
    }
);

const backgrounds_store = new Store(
    (hour: number) => new Promise<string>(
        (resolve, reject) => {
            const background_path = `backgrounds/${hour}.png`;

            const loader = new Image();
            loader.onload = () => { resolve(background_path); };
            loader.onerror = reject;
            loader.src = background_path;
        }
    )
);

const load_song = async (hour?: number) => {
    const current_hour = hour ?? Clock.current_hour();

    return Promise.all(
        [
            backgrounds_store.get(current_hour),
            songs_store.get(current_hour)
        ]
    );
};

const blurred_background = document.getElementById("blurred-background")! as HTMLImageElement;
const center_card = document.getElementById("center-card")! as HTMLImageElement;

const set_background = (background_path: string) => {
    blurred_background.src = background_path;
    center_card.src = background_path;
};

(async () => {
    const [background, _] = await load_song();
    set_background(background);
})();

gsap.ticker.lagSmoothing(0);

const require_interaction = document.getElementById("require-interaction")! as HTMLDivElement;

require_interaction.onclick = () => {
    require_interaction.onclick = () => { };
    require_interaction.classList.remove("click");

    const FADE_IN_DURATION = 3 as Seconds;

    (async () => {
        if (Clock.to_next_hour_s() <= FADE_IN_DURATION) {
            load_song(Clock.next_hour()); // This is intentionally not [await]ed. We're just preloading here. 

            const black_center_card = require_interaction.firstElementChild;

            gsap.to(
                black_center_card,
                {
                    opacity: 0,
                    duration: Clock.to_next_hour_s(),
                    ease: "expo.out",
                }
            );

            const SAFETY_BUFFER_MS = 5 as Milliseconds;

            await delay((Clock.to_next_hour() + SAFETY_BUFFER_MS) as Milliseconds);
        }

        const [background, song] = await load_song();

        set_background(background);

        gsap.to(
            require_interaction,
            {
                opacity: 0,
                duration: FADE_IN_DURATION,
                ease: "expo.out",
            }
        );

        setTimeout(
            () => require_interaction.remove(),
            Clock.to_ms(FADE_IN_DURATION)
        );

        const now = context.currentTime;

        fade_in.gain.setValueAtTime(0.01, now);
        fade_in.gain.exponentialRampToValueAtTime(1, now + FADE_IN_DURATION);

        song.schedule_play(rhythm.current_time, Clock.to_s(Clock.since_hour_start()) as Seconds);

        const fader = document.getElementById("fader")! as HTMLDivElement;

        while (true) {
            const BUFFER_MILLISECONDS = (MINUTE_MS * 2) as Milliseconds;

            await delay((Clock.to_next_hour() - BUFFER_MILLISECONDS) as Milliseconds);

            const [next_background, next_song] = await load_song(Clock.next_hour());

            next_song.schedule_play(
                (rhythm.current_time + Clock.to_next_hour_s()) as Seconds
            );

            const to_next_hour = Clock.to_next_hour_s();
            const fade_out_duration = Math.min(to_next_hour, FADE_OUT_DURATION);

            gsap.to(
                fader,
                {
                    opacity: 1,
                    duration: fade_out_duration,
                    ease: "power3.out",
                    delay: to_next_hour - fade_out_duration
                }
            );

            setTimeout(
                () => set_background(next_background),
                Clock.to_next_hour()
            );

            gsap.to(
                fader,
                {
                    opacity: 0,
                    duration: hour_bells.duration,
                    ease: "expo.in",
                    delay: Clock.to_next_hour_s(),
                }
            );

            await delay((Clock.to_next_hour() + BUFFER_MILLISECONDS) as Milliseconds);
        }
    })();
};