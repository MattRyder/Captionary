import * as ActionTypes from '../constants/ActionTypes';

const InitialState = {
    socketHandle: null,
};

const WebSocketReducer = (state = InitialState, action) => {
    switch(action.type) {
        case ActionTypes.WEBSOCKET_INITIALIZED_ACTION:
            state.socketHandle = action.payload.socketHandle;
            return state;
        default:
            return state;
    }
};

export default WebSocketReducer;