import { CharacterPayload } from "entities/api";
import lemonade from "lemonadejs";
import { SSE } from "shared/lib";

function VeinSelectedCharacter() {
    this.onconnect = () => {
        SSE.on('selected', this, (selected: CharacterPayload) => {
            if (!selected) return;
            this.occupation = selected.player_character_data.human_occupation
            this.name = selected.player_character_data.name;
            this.stats = selected.stats;
            this.inventory = selected.inventory;
        });
    }

    this.ondisconnect = (self: any) => SSE.disconnect(self);

    return (render: any) => render`
        <section class="${this.selected ? 'flex flex-col gap-4 rounded overflow-hidden shadow-lg p-6 bg-gray-800' : 'hidden'}">
            <vein-character-data 
                class="flex gap-4" 
                name=${this.name}
                occupation=${this.occupation}
            />
            <div class="flex gap-[12rem]">
                <section class="flex flex-col gap-4">
                    <h3 class="text-lg font-bold">Stats</h3>
                    <div class="flex flex-col gap-4" :loop=${this.stats}>
                        <vein-stat name={{self.human_name}} value={{self.value}} />
                    </div>
                </section>
                <section class="flex flex-col gap-4">
                    <h3 class="text-lg font-bold">Inventory</h3>
                    <div class="flex flex-col gap-4" :loop=${this.inventory}>
                        <vein-item name={{self.name}} stack={{self.stack}} />
                    </div>
                </section>
            </div>
        </section>`;
}
lemonade.createWebComponent('selected-character', VeinSelectedCharacter, { prefix: 'vein' })
