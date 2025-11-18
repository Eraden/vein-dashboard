import lemonade from "lemonadejs";
import { SSE } from "shared/lib";
import "./vein-character";
import "./vein-player-stats";
import "./vein-players";
import "./vein-selected-character";
import "./vein-uptime";
import "./vein-character-data";
import "./vein-item";
import "./vein-player-stat";
import "./vein-player";
import "./vein-stat";

function StatusPage() {
    const page = lemonade.state('status');

    this.ondisconnect = SSE.disconnect;
    this.onconnect = (self: any) =>
        SSE.on('page', self, (current: string) => {
            page.value = current;
        });
    ;

    return (render: any) => render`
        <article id="status" class="${page.value == 'status' ? 'flex flex-col gap-4 p-8' : 'hidden'}">
            <section id="uptime" class="flex gap-4">
                <vein-uptime uptime="" />
            </section>

            <vein-players />

            <section id="characters" class="flex flex-col gap-4">
            </section>
        </article>
    `;
}

lemonade.createWebComponent('status-page', StatusPage, { prefix: 'vein' });
