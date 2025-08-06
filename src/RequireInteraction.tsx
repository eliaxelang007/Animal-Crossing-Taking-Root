import { Show, createSignal, children, ParentComponent } from "solid-js";

const RequireInteraction: ParentComponent = (props) => {
    const [interacted, set_interacted] = createSignal(false);
    const reactive_children = children(() => interacted() && props.children);

    return (
        <Show
            when={interacted()}
            fallback={
                <div on:click={() => set_interacted(true)}>
                    <h1>"Click to start the player!"</h1>
                </div>
            }
        >
            {reactive_children()}
        </Show>
    );
};

export default RequireInteraction;