export function Component(tag: string) {
    return function(target: any) {
        customElements.define(tag, target);
    }
}

interface EventHandler {
    el: HTMLElement,
    callback?: Function | null | undefined;
}

class ServerSideEvents {
    events: Record<string, EventHandler[]> = {};

    constructor() {
        this.events = {};
    }

    on(msg_type: string, el: HTMLElement, callback?: Function) {
        let queue = this.events[msg_type] || [];
        queue.push({ el, callback });
        this.events[msg_type] = queue;
    }

    disconnect = (target: HTMLElement) => {
        for (const ty in this.events) {
            const idx = this.events[ty].findIndex(({ el }) => {
                return el == target
            });
            if (idx != -1) this.events[ty].splice(idx, 1);
        }
    }
    emit(msg_type: string, payload: any) {
        let queue = this.events[msg_type] || [];
        for (const { el, callback } of queue) {
            const target = el as any;
            // console.log(msg_type, {
            //     el,
            //     different: !_.isEqual(el[msg_type], payload),
            //     payload: structuredClone(payload),
            //     value: el[msg_type]
            // });
            if (!_.isEqual(target[msg_type], payload)) {
                callback && callback(payload);
                target[msg_type] = payload;
            }
        }
    }
}

export const SSE = new ServerSideEvents();
export const CHARACTER_SELECTED = 'Character:Selected';
