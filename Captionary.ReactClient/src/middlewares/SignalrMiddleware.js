import { push } from 'react-router-redux';
import { HubConnectionBuilder, LogLevel } from '@aspnet/signalr';
import { SignalrConnectAction } from '../actions/SignalrConnectAction';
import * as ActionTypes from '../constants/ActionTypes';
import * as SignalrActions from '../constants/SignalrActions';
import { GameAccessResponseAction } from '../actions/GameAccessResponseAction';
import { ReceiveChatMessageAction } from '../actions/ReceiveChatMessageAction';
import { RoundStartedAction } from '../actions/RoundStartedAction';

const SERVER_HOST = process.env.REACT_APP_SIGNALR_HOST;

export const SignalrMiddleware = (store) => {
    return (next) => (action) => {
        switch (action.type) {
            case ActionTypes.GAME_REQUEST_ACCESS_ACTION:
                const hubConnection = store.getState().game.hubConnection;

                hubConnection
                    .invoke(SignalrActions.SERVER_ACTION_LOGIN, action.payload.playerName, action.payload.roomId)
                    .catch(err => { console.error("Failed to login: " + err) });
                break;
            case ActionTypes.GAME_ACCESS_RESPONSE_ACTION:
                store.dispatch(push("/play"));
                break;
            case ActionTypes.SIGNALR_CONNECT_ACTION: {
                console.log("SignalR Middleware: Connecting to Game Hub...");
                break;
            }
            case ActionTypes.SEND_CHAT_MESSAGE_ACTION: {
                const hubConnection = store.getState().game.hubConnection;
                const msg = action.payload.message;
                console.log(msg.senderName + " sent message: " + msg.message);

                hubConnection
                    .invoke(SignalrActions.SERVER_ACTION_SEND_MESSAGE, msg)
                    .catch(err => { console.error("Failed to send message: " + err) });
                break;
            }
            default:
                break;
        }

        return next(action);
    }
};

export const SignalrInit = (store) => {
    const hubConnection = new HubConnectionBuilder()
        .withUrl(SERVER_HOST)
        .configureLogging(LogLevel.Trace)
        .build();

    hubConnection.on(SignalrActions.SERVER_ACTION_JOIN_GAME, (playerName, roomId) => {
        store.dispatch(GameAccessResponseAction(playerName, roomId));
    });

    hubConnection.on(SignalrActions.SERVER_ACTION_RECEIVE_MESSAGE, (messageData) => {
        store.dispatch(ReceiveChatMessageAction(messageData.senderId, messageData.senderName, messageData.message));
    });

    hubConnection.on(SignalrActions.SERVER_ACTION_ROUND_STARTED, (roundId, imageUrl) => {
        store.dispatch(RoundStartedAction(roundId, 0xABBA, imageUrl));
    });

    const onError = () => { store.dispatch(push("/error/1000")) };
    store.dispatch(SignalrConnectAction(hubConnection, onError));
};