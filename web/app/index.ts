import lemonade from "lemonadejs";
import { SSE } from "shared/lib";

function VeinApp() {
    let state = lemonade.state('authorized');
    SSE.on('forbidden', this, () => {
        console.log('set to forbidden');
        state.value = 'forbidden';
    });
    SSE.on('authorized', this, () => {
        console.log('set to authorized');
        state.value = 'authorized';
    });
    return (render: any) => render`
        <div class=${state.value}>
            <vein-authorized />
            <vein-login-page />
        </div>
    `;
}

lemonade.createWebComponent('app', VeinApp, { shadowRoot: false, prefix: 'vein' });
