/**
* @typedef CharacterPayload
* @property {string} id
* @property {Stat[]} stats 
* @property {Inventory[]} inventory
* @property {PlayerCharacterData} player_character_data
*/
/**
* @typedef PlayerCharacterData
* @property {string} id
* @property {string} name
* @property {string} occupation
* @property {string} human_occupation
*/
/**
* @typedef PlayerPayload
* @property {string} id
* @property {CharacterPayload[]} characters 
*/
/**
* @typedef Stat
* @property {string} name
* @property {string} human_name
* @property {number} value
*/
/**
* @typedef StatusPayload
* @type {object}
* @property {number} uptime
* @property {Object.<string, PlayerStatusPayload>} online_players
*/
/**
* @typedef PlayerStatusPayload
* @property {string} character_id
* @property {number} time_connected
* @property {string} name
* @property {string} status
*/
/**
* @typedef LogsPayload
* @property {LogLinePayload[]} logs 
*/
/**
* @typedef LogLinePayload
* @property {string} line
* @property {string|null} level
*/

const CHARACTER_SELECTED = 'Character:Selected';

const { set, get, dispatch } = lemonade;

const updateAt = (selector, html) => document.body.querySelector(selector).innerHTML = html;

/**
* @param {CharacterPayload} character
*/
const characterName = (character) => character.player_character_data.name;

const mountTabsHandlers = () => {
    const articles = Array.from(document.body.querySelectorAll('article'));
    Array.from(document.body.querySelectorAll("nav li a")).forEach(el => {
        el.addEventListener('click', (ev) => {
            ev.preventDefault();
            ev.stopPropagation();

            for (const el of articles) el.classList.add('hidden');
            document.body.querySelector(ev.target.href).classList
        });
    });
};

let SSE = {
    events: {},
    /**
    * @param {string} msg_type
    * @param {HTMLElement} el
    * @param {Function} callback
    */
    on: function(msg_type, el, callback) {
        let queue = this.events[msg_type] || [];
        queue.push({ el, callback });
        this.events[msg_type] = queue;
    },
    /**
    * @param {HTMLElement} el
    */
    disconnect: function(targetElement) {
        for (const ty in this.events) {
            const idx = this.events[ty].findIndex(({ el }) => el == targetElement);
            if (idx != -1) this.events[ty].splice(idx, 1);
        }
    },
    fire: function(msg_type, payload) {
        let queue = this.events[msg_type] || [];
        for (const { el, callback } of queue) {
            // console.log(msg_type, {
            //     el,
            //     different: !_.isEqual(el[msg_type], payload),
            //     payload: structuredClone(payload),
            //     value: el[msg_type]
            // });
            if (!_.isEqual(el[msg_type], payload)) {
                callback && callback(payload);
                el[msg_type] = payload;
            }
        }
    }
};

class VeinPlayers extends HTMLElement {
    constructor() {
        super();
        /**
        * @type {PlayerPayload[]}
        */
        this.players = [];
        /**
        * @type {CharacterPayload|null}
        */
        this.selected = null;
        this.selectedClass = 'hidden';
    }

    connectedCallback() {
        SSE.on('players', this);
        SSE.on('selected', this, (character) => {
            this.selectedClass = character ? 'w-2/3' : 'hidden';
        });
        if (!this.el) lemonade.render(this.render, this, this);
    }

    disconnectedCallback() {
        SSE.disconnect(this)
    }

    render() {
        return render => render`
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
}

class VeinSelectedCharacter extends HTMLElement {
    constructor() {
        super();
        /** @type {CharacterPayload} */
        this.selected = null;
    }

    render() {
        return render => render`
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

    connectedCallback() {
        SSE.on('selected', this, (selected) => {
            if (!selected) return;
            this.occupation = selected.player_character_data.human_occupation
            this.name = selected.player_character_data.name;
            this.stats = selected.stats;
            this.inventory = selected.inventory;
        });
        if (!this.el) lemonade.render(this.render, this, this);
    }

    disconnectedCallback() {
        SSE.disconnect(this);
    }
}

class VeinInventoryItem extends HTMLElement {
    constructor() {
        super();
    }

    render() {
        return render => render`
            <div class="flex gap-4">
                <div class="w-[25rem] text-base text-white">${this.name}</div>
                <div class="text-base font-bold text-gray-300">${this.stack}</div>
            </div>
        `;
    }

    connectedCallback() {
        if (!this.el) lemonade.render(this.render, this, this);
    }

    disconnectedCallback() {
        SSE.disconnect(this);
    }
}

class VeinPlayer extends HTMLElement {
    constructor() {
        super();
        /** @type {PlayerPayload} */
        this.player = { characters: [], stats: { stats: [] } };
    }

    render() {
        return render => render`
            <section class="flex flex-col gap-4 rounded overflow-hidden shadow-lg p-6 bg-gray-800">
                <h3 class="font-bold text-xl mb-2">${this.player.last_name}</h3>
                <vein-player-stats stats=${this.player.stats.stats} />

                <div :loop=${this.player.characters} class="flex flex-col gap-4">
                    <vein-character character={{self}} />
                </div>
            </section>`;
    }

    connectedCallback() {
        if (!this.el) lemonade.render(this.render, this, this);
    }
}

class VeinPlayerStats extends HTMLElement {
    constructor() {
        super();
        /** @type {Stat[]} */
        this.stats = [];
    }
    connectedCallback() {
        if (!this.el) lemonade.render(this.render, this, this);
    }
    render() {
        return render => render`
            <details class="flex flex-col gap-4">
                <summary>Player stats</summary>
                <div class="flex flex-col gap-4" :loop=${this.stats}>
                    <vein-player-stat name={{self.human_name}} value={{self.value}} />
                </div>
            </details>
        `;
    }
}
class VeinPlayerStat extends HTMLElement {
    constructor() {
        super();
        this.name = '';
        this.value = 0;
    }
    connectedCallback() {
        if (!this.el) lemonade.render(this.render, this, this);
    }
    render() {
        return render => render`
            <div class="flex gap-4">
                <div class="w-[20rem]">${this.name}</div>
                <div>${this.value}</div>
            </div>
        `;
    }
}

class VeinCharacter extends HTMLElement {
    constructor() {
        super();
        /** @type {CharacterPayload} */
        this.character = { stats: [], player_character_data: {} };
    }
    connectedCallback() {
        SSE.on('online_characters', this, online => {
            console.log(online, this.character.id);
            this.isOnline = online.includes(this.character.id);
            console.log(this.isOnline);
        });
        if (!this.el) lemonade.render(this.render, this, this);
    }
    render() {
        const selectCharacter = (ev) => {
            ev.preventDefault();
            ev.stopPropagation();
            SSE.fire('selected', this.character);
            dispatch(CHARACTER_SELECTED, this.character);
        }

        return render => render`
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
}

class VeinStat extends HTMLElement {
    constructor() {
        super();
        this.name = '';
        this.value = 0;
    }
    connectedCallback() {
        if (!this.el) lemonade.render(this.render, this, this);
    }
    render() {
        return render => render`
            <div class="flex gap-4">
                <div class="w-[20rem] text-base text-white">${this.name}</div>
                <div class="text-base font-bold text-gray-300">${this.value}</div>
            </div>
        `;
    }
}

class VeinCharacterData extends HTMLElement {
    constructor() {
        super();
        this.name = '';
        this.occupation = '';
    }
    connectedCallback() {
        if (!this.el) lemonade.render(this.render, this, this);
    }
    render() {
        return render => render`
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
}

class VeinUptime extends HTMLElement {
    constructor() {
        super();
        this.uptime = 0;
    }

    render() {
        return render => render`
            <div class="flex gap-4">
                <h1>Uptime:</h1>
                <div>${this.uptime}<div>
            </div>
        `;
    }
    connectedCallback() {
        SSE.on('uptime', this);
        if (!this.el) lemonade.render(this.render, this, this);
    }

    disconnectedCallback() {
        SSE.disconnect(this)
    }
}

class VeinLogs extends HTMLElement {
    constructor() {
        super();
        this.logs = '';
    }

    render() {
        return render => render`
        `;
    }
    connectedCallback() {
        SSE.on('logs', this);
        if (!this.el) lemonade.render(this.render, this, this);
    }

    disconnectedCallback() {
        SSE.disconnect(this)
    }
}

class VeinMenu extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        if (this.el) return;
        lemonade.render(this.render, this, this);
    }

    render() {
        const status = (ev) => {
            ev.preventDefault();
            ev.stopPropagation();
            SSE.fire('page', 'status');
        };
        const logs = (ev) => {
            ev.preventDefault();
            ev.stopPropagation();
            SSE.fire('page', 'logs');
        };
        return render => render`
        <nav class="bg-gray-800 p-6">
            <ul class="mt-8 flex gap-4">
                <li>
                    <a href="#status" class="text-gray-400 hover:text-white" onclick=${status}>
                        Status
                    </a>
                </li>
                <li>
                    <a href="#logs" class="text-gray-400 hover:text-white" onclick=${logs}>
                        Logs
                    </a>
                </li>
            </ul>
        </nav>
        `;
    }
}

class StatusPage extends HTMLElement {
    constructor() {
        super();
        this.page = 'status';
    }
    connectedCallback() {
        SSE.on('page', this, page => console.log(page));
        if (!this.el) lemonade.render(this.render, this, this);
    }

    disconnectedCallback() {
        SSE.disconnect(this);
    }
    render() {
        return render => render`
            <article id="status" class="${this.page == 'status' ? 'flex flex-col gap-4 p-8' : 'hidden'}">
                <section id="uptime" class="flex gap-4">
                    <vein-uptime uptime=0 />
                </section>

                <vein-players />

                <section id="characters" class="flex flex-col gap-4">
                </section>
            </article>
        `;
    }
}

class LogsPage extends HTMLElement {
    constructor() {
        super();
        this.page = 'status';
    }
    connectedCallback() {
        SSE.on('page', this);
        SSE.on('logs', this);
        if (this.el) return;
        lemonade.render(this.render, this, this);
    }
    disconnectedCallback() {
        SSE.disconnect(this);
    }
    render() {
        return render => render`
            <article id="logs" class="${this.page == 'logs' ? 'flex flex-col gap-4 p-8' : 'hidden'}">
                <div :loop=${this.logs}>
                    <vein-log-line line={{self.line}} level={{self.level}} />
                </div>
            </article>
        `;
    }
}

class LogLineView extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        if (this.el) return;
        lemonade.render(this.render, this, this);
    }
    disconnectedCallback() {
        SSE.disconnect(this);
    }
    render() {
        return render => render`
            <div class="${this.level == "error" ? 'text-red-800' : this.level == 'warn' ? 'text-yellow-600' : ''}">
                ${this.line}
            </div>
        `;
    }
}

document.addEventListener('DOMContentLoaded', () => {
    const handleEvent = (event) => {
        const payload = JSON.parse(event.data);
        for (const eventName in payload) {
            const handler = HANDLERS[eventName];
            if (handler) handler(payload[eventName]);
        }
    };

    const sse = new EventSource("/events");
    sse.onopen = () => console.log("SSE opened");
    sse.addEventListener("server_event", ev => handleEvent(ev));
    sse.onerror = (e) => {
        console.error(e);
        if (e.readyState === EventSource.CLOSED) {
            console.log("DEBUG: eventSource connection CLOSED! Trying to reconnect!");
        }
    };
});

customElements.define('vein-menu', VeinMenu);
customElements.define('vein-players', VeinPlayers);
customElements.define('vein-player', VeinPlayer);
customElements.define('vein-character', VeinCharacter);
customElements.define('vein-uptime', VeinUptime);
customElements.define('vein-stat', VeinStat);
customElements.define('vein-item', VeinInventoryItem);
customElements.define('vein-character-data', VeinCharacterData);
customElements.define('vein-player-stats', VeinPlayerStats);
customElements.define('vein-player-stat', VeinPlayerStat);
customElements.define('vein-selected-character', VeinSelectedCharacter);
customElements.define('vein-status-page', StatusPage);
customElements.define('vein-logs-page', LogsPage);
customElements.define('vein-log-line', LogLineView);

const HANDLERS = {
    /**
    * @param {PlayerPayload[]} players
    */
    players: (players) => {
        SSE.fire('players', players);
        SSE.fire('characters', players.map(p => p.characters).flat());
    },
    /**
    * @param {StatusPayload} status
    */
    status: (status) => {
        SSE.fire('status', status);
        SSE.fire('online_characters', Object.values(status.online_players).map(p => p.character_id));
        SSE.fire('uptime', status.uptime);
    },
    logs: (logs) => {
        SSE.fire('logs', logs);
    },
};
