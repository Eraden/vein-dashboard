import lemonade from "lemonadejs";

function VeinPlayerStat() {
    return (render: any) => render`
        <div class="flex gap-4">
            <div class="w-[20rem] justify-left">${this.name}</div>
            <div class="justify-right">${(this.value || 0).toFixed(2)}</div>
        </div>
    `;
}

lemonade.createWebComponent('player-stat', VeinPlayerStat, { prefix: 'vein' })
