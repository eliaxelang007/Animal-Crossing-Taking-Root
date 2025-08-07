import { Play, Sequence, Repeat, RhythmContext, Seconds, Gain, Attached, AnyCompiledCommand } from "rhythm.js";
import { clock, HOUR_SECONDS, Milliseconds, MINUTE_MILLISECONDS } from "./clock";
import { gsap } from "gsap";

const require_interaction = document.getElementById("require-interaction")! as HTMLDivElement;
const blurred_background = document.getElementById("blurred-background")! as HTMLImageElement;
const center_card = document.getElementById("center-card")! as HTMLImageElement;

const delay = (wait: Milliseconds) => new Promise((resolve) => setTimeout(resolve, wait));

const context = new AudioContext();
const fade_in = context.createGain();

fade_in.connect(context.destination);

const rhythm = new RhythmContext(context);

const FADE_OUT_DURATION = 20 as Seconds;

const hour_bells = await rhythm.compile(new Play("audios/hour_bells.oga"));
const hour_song_builders = Array
    .from({ length: 24 }, (_, hour) => hour)
    .map(
        (hour) => {
            const build_track = async () => {
                const hour_song = await rhythm.compile(
                    new Gain(
                        {
                            gain_keyframes: [
                                {
                                    transition: undefined,
                                    value: 1,
                                    from_start: (HOUR_SECONDS - FADE_OUT_DURATION) as Seconds
                                },
                                {
                                    transition: "exponential",
                                    value: 0.01, // Can't exponentially fade out to a flat 0.
                                    from_start: HOUR_SECONDS
                                }
                            ]
                        },
                        new Sequence(
                            [
                                hour_bells,
                                new Repeat(
                                    {
                                        duration: (HOUR_SECONDS - hour_bells.duration) as Seconds
                                    },
                                    new Play(`audios/${hour}.oga`)
                                )
                            ]
                        )
                    )
                );

                return hour_song.attach_to(fade_in);
            };

            let track_builder: Promise<Attached<GainNode, AnyCompiledCommand>> | null = null;

            return () => {
                if (track_builder === null) {
                    track_builder = build_track();
                }

                return track_builder;
            };
        }
    );

const set_background = (hour: number) => {
    const src = `backgrounds/${hour}.png`;
    blurred_background.src = src;
    center_card.src = src;
};

const current_hour = clock.current_hour();

set_background(current_hour);
(async () => { await hour_song_builders[current_hour](); })();

require_interaction.onclick = () => {
    require_interaction.remove();

    const play = async () => {
        const current_hour = clock.current_hour();
        const hour_song = await hour_song_builders[current_hour]();

        const now = context.currentTime;

        const FADE_IN_DURATION = 3 as Seconds;

        fade_in.gain.setValueAtTime(0.01, now);
        fade_in.gain.exponentialRampToValueAtTime(1, now + FADE_IN_DURATION);

        hour_song.schedule_play(rhythm.current_time, (clock.since_hour_start() / 1000) as Seconds);

        while (true) {
            const FIVE_MINUTES_MILLISECONDS = MINUTE_MILLISECONDS * 5;

            await delay((clock.to_next_hour() - FIVE_MINUTES_MILLISECONDS) as Milliseconds);

            const next_hour = (clock.current_hour() + 1) % 24;

            const next_hour_song = await hour_song_builders[next_hour]();

            next_hour_song.schedule_play(
                (rhythm.current_time + (clock.to_next_hour() / 1000)) as Seconds
            );

            gsap.to(
                ".fader",
                {
                    opacity: 1,
                    duration: FADE_OUT_DURATION,
                    ease: "expo",
                    delay: (clock.to_next_hour() / 1000) - FADE_OUT_DURATION,
                    onComplete: () => {
                        set_background(next_hour);
                    }
                }
            );

            gsap.to(
                ".fader",
                {
                    opacity: 0,
                    duration: hour_bells.duration,
                    ease: "expo",
                    delay: (clock.to_next_hour() / 1000)
                }
            );

            await delay((clock.to_next_hour() + FIVE_MINUTES_MILLISECONDS) as Milliseconds);
        }
    };

    play();
};