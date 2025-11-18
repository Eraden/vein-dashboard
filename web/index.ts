import "./app";
import "./pages";
import "./features";

import { SSE } from "./shared/lib";
import { LogsPayload, PlayerPayload, PlayerStatusPayload, StatusPayload } from "./entities/api";

document.addEventListener('DOMContentLoaded', () => {
    const handleEvent = (event: MessageEvent) => {
        const payload = JSON.parse(event.data);
        for (const event_name in payload) {
            const handler = (HANDLERS as any)[event_name as string];
            if (handler) handler(payload[event_name]);
        }
    };

    const sse = new EventSource("/events");
    sse.onopen = () => console.log("SSE opened");
    sse.addEventListener("server_event", ev => handleEvent(ev));
    sse.addEventListener("forbidden", () => {
        console.log('forbidden');
        SSE.emit('forbidden', null);
        sse.close();
    });
    sse.onerror = (e: any) => {
        console.error(e);
        if (e.readyState === EventSource.CLOSED) {
            console.log("DEBUG: eventSource connection CLOSED! Trying to reconnect!");
        } else {
            sse.close();
        }
    };
});

const HANDLERS = {
    players: (players: PlayerPayload[]) => {
        SSE.emit('players', players);
        SSE.emit('characters', players.map(p => p.characters).flat());
    },
    status: (status: StatusPayload) => {
        SSE.emit('status', status);
        SSE.emit('online_characters', Object.values(status.online_players).map((p: PlayerStatusPayload) => p.character_id));
        SSE.emit('uptime', status.human_uptime);
    },
    logs: (logs: LogsPayload) => {
        SSE.emit('logs', logs);
    },
};
