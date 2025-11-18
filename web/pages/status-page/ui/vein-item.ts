import lemonade from "lemonadejs";

function VeinInventoryItem(children: any, { onchange }) {
    return (render: any) => render`
        <div class="flex gap-4">
            <div class="w-[25rem] text-base text-white">${this.name}</div>
            <div class="text-base font-bold text-gray-300">${this.stack}</div>
        </div>
    `;
}

lemonade.createWebComponent('item', VeinInventoryItem, { prefix: 'vein' });
