import { GAME_REQUEST_ACCESS_ACTION } from '../constants/ActionTypes';

export const GameRequestAccessAction = (playerName, roomId) => ({
    type: GAME_REQUEST_ACCESS_ACTION,
    payload: {
        playerName: playerName,
        roomId: roomId
    }
});