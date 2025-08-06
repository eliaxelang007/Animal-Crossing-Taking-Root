/* @refresh reload */
import { render } from 'solid-js/web';
import { Router, Route } from "@solidjs/router";

import './index.css';
import Player from './Player';
import RequireInteraction from './RequireInteraction';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() => (
  <Router>
    <Route path="/" component={
      () => {
        return (
          <RequireInteraction>
            <Player />
          </RequireInteraction>
        );
      }
    } />
  </Router>
), root!);
