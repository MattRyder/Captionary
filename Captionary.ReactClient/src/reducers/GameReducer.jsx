import * as ActionTypes from '../constants/ActionTypes';

const InitialState = {
    hubConnection: null,
    chatMessages: [],
    sessionInfo: {
        playerName: "",
        roomId: 0
    }
};

const GameReducer = (state = InitialState, action) => {
    switch (action.type) {
        case ActionTypes.LOGIN_ACTION: {
            state.sessionInfo = action.payload;
            break;
        }
        case ActionTypes.SIGNALR_CONNECT_ACTION: {
            state.hubConnection = action.payload.hubConnection;
            state.hubConnection
                .start()
                .catch(err => action.payload.onErrorCallback());
            break;
        }
        case ActionTypes.RECEIVE_CHAT_MESSAGE_ACTION: {
            return {
                ...state, chatMessages:[...state.chatMessages, action.payload.message]
            };
        }
        default:
            break;
    }

    return state;
};

export default GameReducer;