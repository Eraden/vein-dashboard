import lemonade from "lemonadejs";

function VeinStat() {
    return (render: any) => render`
        <div class="flex gap-4">
            <div class="w-[20rem] text-base text-white">${this.name}</div>
            <div class="text-base font-bold text-gray-300">${(this.value || 0).toFixed(2)}</div>
        </div>
    `;
}

lemonade.createWebComponent('stat', VeinStat, { prefix: 'vein' });
