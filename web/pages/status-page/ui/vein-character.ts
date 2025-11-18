import lemonade from "lemonadejs";
import { CHARACTER_SELECTED, SSE } from "shared/lib";

function VeinCharacter() {
    this.isOnline = false;
    this.onconnect = () => {
        SSE.on('online_characters', this, (online: string[]) => {
            this.isOnline = online.includes(this.character.id);
        });
    };
    this.ondisconnect = SSE.disconnect;

    const selectCharacter = (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();
        SSE.emit('selected', this.character);
        lemonade.dispatch(CHARACTER_SELECTED, this.character);
    };

    return (render: any) => render`
        <a class="flex gap-4 items-center cursor-pointer" onclick=${selectCharacter}>
            <svg class="w-4 h-4" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M7.5 0C3.35786 0 0 3.35786 0 7.5C0 11.6421 3.35786 15 7.5 15C11.6421 15 15 11.6421 15 7.5C15 3.35786 11.6421 0 7.5 0Z" class="${this.isOnline ? 'fill-lime-600' : 'fill-gray-400'}"/>
            </svg>
            <vein-character-data 
                class="flex gap-4" 
                name=${this.character.player_character_data.name}
                occupation=${this.character.player_character_data.human_occupation}
            />
        </a>
        `
}
lemonade.createWebComponent("character", VeinCharacter, { prefix: 'vein' });
