import lemonade from "lemonadejs";

function VeinPlayerStats() {
    return (render: any) => render`
        <details class="flex flex-col gap-4">
            <summary>Player stats</summary>
            <div class="flex flex-col gap-4" :loop=${(this.stats || [])}>
                <vein-player-stat name={{self.human_name}} value={{self.value}} />
            </div>
        </details>
    `;
}

lemonade.createWebComponent('player-stats', VeinPlayerStats, { prefix: 'vein' })
