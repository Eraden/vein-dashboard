import lemonade from "lemonadejs";
import { SSE } from "shared/lib";

function LogsPage() {
    const page = lemonade.state('status');
    const logs = lemonade.state([]);

    this.onconnect = (self: any) => {
        SSE.on('page', self, (value: any) => page.value = value);
        SSE.on('logs', self, (value: any) => page.value = value);
    };
    this.ondisconnect = (self: any) => SSE.disconnect(self);

    return (render: any) => render`
        <article id="logs" class="${page.value == 'logs' ? 'flex flex-col gap-4 p-8' : 'hidden'}">
            <div :loop=${logs.value}>
                <vein-log-line line={{self.line}} level={{self.level}} />
            </div>
        </article>
    `;
}

lemonade.createWebComponent('logs-page', LogsPage, { prefix: 'vein' })
