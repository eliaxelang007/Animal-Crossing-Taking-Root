import { createSignal, onMount, type Component } from 'solid-js';

import { Milliseconds } from './clock';
import { Seconds, Play, Repeat, RhythmContext, CompiledPlay } from 'rhythm.js';

const Player: Component = () => {
  const [background, set_background] = createSignal("backgrounds/15.png");

  onMount(async () => {
    const delay = (wait: Milliseconds) => new Promise((resolve) => setTimeout(resolve, wait));

    const rhythm = new RhythmContext();

    const hour_bells = await rhythm.compile(new Play("/src/assets/audios/hour_bells.oga"));

    const repeating = await rhythm.compile_attached(new Repeat(
      {
        duration: 60 as Seconds
      },
      hour_bells,
    ));

    repeating.schedule_play(0 as Seconds);

    await delay((31 * 1000) as Milliseconds);
  });

  return (
    <img class="fill-parent img-cover" src={background()}>

    </img>
  );
};

export default Player;
