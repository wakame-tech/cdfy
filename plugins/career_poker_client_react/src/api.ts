export interface Room {
    room_id: string,
    users: string[],
    states: Record<string, string>,
}

const API_ORIGIN = `http://localhost:1234`;

const map_promise = <T>(res: Response): Promise<T> => {
    return res.json()
        .then(res => {
            if ('error' in res) {
                throw res
            }
            return res
        })
}

export const fetchRoom = async (roomId: string, userId: string): Promise<Room> => {
    const res = await fetch(`${API_ORIGIN}/rooms/${roomId}`);
    return map_promise(res);
}

export const joinRoom = async (roomId: string, userId: string): Promise<Room> => {
    const res = await fetch(`${API_ORIGIN}/rooms/${roomId}/join/${userId}`, {
        method: 'POST',
    });
    return map_promise(res);
}

export const createRoom = async (roomId: string): Promise<Room> => {
    const res = await fetch(`${API_ORIGIN}/rooms/${roomId}`, { method: 'POST' });
    return map_promise(res);
}

export const loadPlugin = async (roomId: string): Promise<Room> => {
    const res = await fetch(`${API_ORIGIN}/rooms/${roomId}/plugins/counter`, { method: 'POST' });
    return map_promise(res);
}

export const sendMessage = async <T>(roomId: string, userId: string, message: T): Promise<Room> => {
    const res = await fetch(`${API_ORIGIN}/rooms/${roomId}/plugins/counter/message`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            user_id: userId,
            message,
        })
    });
    return map_promise(res);
}