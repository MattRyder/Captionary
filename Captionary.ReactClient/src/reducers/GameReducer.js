import * as ActionTypes from '../constants/ActionTypes';

const InitialState = {
    hubConnection: null,
    chatMessages: [],
    sessionInfo: {
        playerName: "",
        roomId: ""
    }
};

const GameReducer = (state = InitialState, action) => {
    switch (action.type) {
        case ActionTypes.GAME_ACCESS_RESPONSE_ACTION:
            console.log(
                `Welcome to Room ${action.payload.roomId}, ${action.payload.playerName}`);
            return Object.assign({}, state, {
                sessionInfo: action.payload
            });
        case ActionTypes.SIGNALR_CONNECT_ACTION: {
            state.hubConnection = action.payload.hubConnection;
            state.hubConnection
                .start()
                .catch(err => action.payload.onErrorCallback());
            return state;
        }
        case ActionTypes.RECEIVE_CHAT_MESSAGE_ACTION: {
            return {
                ...state, chatMessages: [...state.chatMessages, action.payload.message]
            };
        }
        default:
            return state;
    }
};

export default GameReducer;