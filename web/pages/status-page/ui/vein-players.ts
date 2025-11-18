import { CharacterPayload } from "entities/api";
import lemonade from "lemonadejs";
import { SSE } from "shared/lib";

function VeinPlayers() {
    this.selectedClass = 'hidden';

    this.onconnect = (self: any) => {
        SSE.on('players', self);
        SSE.on('selected', self, (character: CharacterPayload) => {
            self.selectedClass = character ? 'w-2/3' : 'hidden';
        });
    };

    this.ondisconnect = SSE.disconnect;

    return (render: any) => render`
        <section id="players" class="flex flex-col gap-4">
            <div class="flex gap-4">
                <div :loop=${this.players} class="${this.selected ? 'flex flex-col gap-4 w-1/3' : 'flex flex-col gap-4 w-full'}">
                    <vein-player player={{self}} />
                </div>
                <vein-selected-character :render=${this.selected} />
            </div
        </section>
    `;
}
lemonade.createWebComponent('players', VeinPlayers, { prefix: 'vein' });
