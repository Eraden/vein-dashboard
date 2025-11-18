import lemonade from "lemonadejs";

function VeinPlayer() {
    return (render: any) => render`
        <section class="flex flex-col gap-4 rounded overflow-hidden shadow-lg p-6 bg-gray-800">
            <h3 class="font-bold text-xl mb-2">${this.player.last_name}</h3>
            <vein-player-stats stats=${this.player.stats.stats} />

            <div :loop=${this.player.characters} class="flex flex-col gap-4">
                <vein-character character={{self}} />
            </div>
        </section>`;
}

lemonade.createWebComponent('player', VeinPlayer, { prefix: 'vein' });
