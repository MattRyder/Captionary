import { GAME_ACCESS_RESPONSE_ACTION } from '../constants/ActionTypes';

export const GameAccessResponseAction = (playerName, roomId) => ({
    type: GAME_ACCESS_RESPONSE_ACTION, 
    payload: {
        playerName: playerName,
        roomId: roomId
    }
});