import { SIGNALR_CONNECT_ACTION } from '../constants/ActionTypes';

export const SignalrConnectAction = (hubConnection = null, onErrorCallback = {}) => ({
    type: SIGNALR_CONNECT_ACTION,
    payload: {
        hubConnection: hubConnection,
        onErrorCallback: onErrorCallback
    }
});