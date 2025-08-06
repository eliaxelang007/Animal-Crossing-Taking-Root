import { Show, createSignal, ParentComponent } from "solid-js";

const RequireInteraction: ParentComponent = (props) => {
    const [interacted, set_interacted] = createSignal(false);

    return (
        <div class="fill-parent stack">
            {props.children}

            {!interacted() && <div
                class="fill-parent blur flex flex-center-children"
                onClick={() => set_interacted(true)}
            >
                <h1>Click here to start playing music!</h1>
            </div>}
        </div>
    );
};

export default RequireInteraction;

// import { Show, createSignal, children, ParentComponent } from "solid-js";

// const RequireInteraction: ParentComponent = (props) => {
//     const [interacted, set_interacted] = createSignal(false);

//     return (
//         <div style={{ "position": "relative" }}>
//             {props.children}
//             <Show
//                 when={interacted()}
//                 fallback={
//                     <div on:click={() => set_interacted(true)}>
//                         <h1>"Click to start the player!"</h1>
//                     </div>
//                 }
//             >
//             </Show>
//         </div>
//     );
// };

// export default RequireInteraction;