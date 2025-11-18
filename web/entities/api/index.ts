export interface CharacterPayload {
    id: string;
    stats: StatPayload[];
    inventory: CharacterInventoryPayload[];
    player_character_data: PlayerCharacterData;
}

export const defaultCharacter = (): CharacterPayload => ({
    id: '',
    stats: [],
    inventory: [],
    player_character_data: {
        id: '',
        name: '',
        occupation: '',
        human_occupation: ''
    }
});

export interface PlayerCharacterData {
    id: string;
    name: string;
    occupation: string;
    human_occupation: string;
}
export interface PlayerPayload {
    id: string;
    characters: CharacterPayload[];
    first_join_time: string,
    last_join_time: string,
    last_name: string,
    server_muted: boolean,
    stats: PlayerStatsPayload,
}

export const defaultPlayer = (): PlayerPayload => ({
    id: '',
    characters: [],
    first_join_time: '',
    last_join_time: '',
    last_name: '',
    server_muted: false,
    stats: { start_time: '', stats: [] }
});

export interface PlayerStatsPayload {
    start_time: string,
    stats: StatPayload[],
}

export interface StatPayload {
    name: string;
    human_name: string;
    value: number;
}
export interface StatusPayload {
    uptime: number;
    human_uptime: string;
    online_players: Record<string, PlayerStatusPayload>;
}
export interface PlayerStatusPayload {
    character_id: string;
    time_connected: number;
    name: string;
    status: string;
}
export interface LogsPayload {
    logs: LogLinePayload[];
}
export interface LogLinePayload {
    line: string;
    level: string | null;
}
export interface CharacterInventoryPayload {
    items: CharacterInventoryItemPayload[],
}
export interface CharacterInventoryItemPayload {
    item: string,
    name: string,
    variant: number,
    item_data: CharacterInventoryItemData,
    acquisition_time: number,
    slot: string,
    inventory: string,
    instance: string,
    damage_perc: number,
    id: string,
    decay: number,
    stack: number,
    custom_label: string,
}

export interface CharacterInventoryItemData {
    data: string[]
}
