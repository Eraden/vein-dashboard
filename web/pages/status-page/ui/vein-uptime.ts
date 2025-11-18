import lemonade from "lemonadejs";
import { SSE } from "shared/lib";

function VeinUptime() {
    const uptime = lemonade.state('');
    this.onconnect = (self: any) => {
        SSE.on('uptime', self, (time: string) => {
            uptime.value = time;
        });
    };

    this.ondisconnect = (self: any) => {
        SSE.disconnect(self)
    };
    return (render: any) => render`
        <div class="flex gap-4">
            <h1>Uptime:</h1>
            <div>${uptime.value}<div>
        </div>
    `;
}

lemonade.createWebComponent('uptime', VeinUptime, { prefix: 'vein' });
