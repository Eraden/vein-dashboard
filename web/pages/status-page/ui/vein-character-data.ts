import lemonade from "lemonadejs";

function VeinCharacterData() {
    return (render: any) => render`
        <div class="flex gap-4 cursor-pointer">
            <div class="font-bold text-xl cursor-pointer">
                ${this.name}
            </div>
            <div class="text-gray-500 text-base cursor-pointer">
                ${this.occupation}
            </div>
        </div>
    `;
}

lemonade.createWebComponent('character-data', VeinCharacterData, { prefix: 'vein' });
