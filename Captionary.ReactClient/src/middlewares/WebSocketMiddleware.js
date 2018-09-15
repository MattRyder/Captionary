import { push } from "react-router-redux";
import * as ActionTypes from "../constants/ActionTypes";
import {
  WebSocketInitializedAction,
  UserLoginResponseAction,
  JoinRoomResponseAction
} from "../actions/WebSocketActions";

import Sockette from "sockette";
const WEBSOCKET_HOST = process.env.REACT_APP_WEBSOCKET_HOST;

const Responses = {
  USER_LOGIN: "UserLoginResponse",
  JOIN_ROOM: "UserJoinedRoomResponse"
};

export const WebSocketMiddleware = store => {
  return next => action => {
    let socketHandle = store.getState().websocket.socketHandle;

    switch (action.type) {
      case ActionTypes.WEBSOCKET_INITIALIZED_ACTION:
        break;
      case ActionTypes.USER_LOGIN_ACTION:
        socketHandle.send(JSON.stringify(action.payload));
        break;
      case ActionTypes.USER_LOGIN_RESPONSE_ACTION:
        break;
      case ActionTypes.JOIN_ROOM_ACTION:
        socketHandle.send(JSON.stringify(action.payload));
        break;
      default:
        console.log("Not covered by WebSocketMiddleware: " + action.type);
        break;
    }

    return next(action);
  };
};

export const WebSocketInit = store => {
  const ws = new Sockette(WEBSOCKET_HOST, {
    timeout: 5e3,
    maxAttempts: 10,
    onopen: e => console.log("Connected!", e),
    onmessage: e => {
      console.log("Received:", e);
      let jsonResponse = JSON.parse(e.data);

      switch (jsonResponse.type) {
        case Responses.USER_LOGIN:
          store.dispatch(UserLoginResponseAction(jsonResponse.user));
          break;
        case Responses.JOIN_ROOM:
          store.dispatch(JoinRoomResponseAction(jsonResponse.room));
          store.dispatch(push("/game"));
          break;
        default:
          console.log("Not covered: " + jsonResponse);
          break;
      }
    },
    onreconnect: e => console.log("Reconnecting...", e),
    onmaximum: e => console.log("Stop Attempting!", e),
    onclose: e => console.log("Closed!", e),
    onerror: e => console.log("Error:", e)
  });

  store.dispatch(WebSocketInitializedAction(ws));
};
