import { onMount, type Component } from 'solid-js';

import { Milliseconds } from './clock';
import { Seconds, Play, Repeat, RhythmContext, CompiledPlay } from 'rhythm.js';

const Player: Component = () => {

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
    <div >
      <header>
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a

          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid
        </a>
      </header>
    </div>
  );
};

export default Player;
