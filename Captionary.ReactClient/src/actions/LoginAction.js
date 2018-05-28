import { LOGIN_ACTION } from '../constants/ActionTypes';

export const LoginAction = (playerName, roomId) => ({
    type: LOGIN_ACTION, 
    payload: {
        playerName: playerName,
        roomId: roomId
    }
});