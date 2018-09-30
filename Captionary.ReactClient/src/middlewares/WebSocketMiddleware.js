import { push } from "react-router-redux";
import * as ActionTypes from "../constants/ActionTypes";
import {
  WebSocketInitializedAction,
  UserLoginResponseAction,
  JoinRoomResponseAction,
  ChatMessageResponseAction,
  GameStartedResponseAction,
  RoundStartedResponseAction,
  CaptionSubmittedResponseAction,
  SubmissionClosedResponseAction
} from "../actions/WebSocketActions";

import Sockette from "sockette";
const WEBSOCKET_HOST = process.env.REACT_APP_WEBSOCKET_HOST;

// Sockette consts
const ECONNREFUSED = 1006;

const Responses = {
  USER_LOGIN: "UserLoginResponse",
  JOIN_ROOM: "UserJoinedRoomResponse",
  CHAT_MESSAGE: "ChatMessageResponse",
  GAME_STARTED: "GameStartedResponse",
  ROUND_STARTED: "RoundStartResponse",
  SUBMIT_CAPTION: "CaptionSubmittedResponse",
  SUBMISSION_CLOSED: "SubmissionClosedResponse"
};

const authenticateMessage = (payload, accessToken) => {
  let firstKey = Object.keys(payload)[0];
  if(firstKey) {
    payload[firstKey]["access_token"] = accessToken;
  }

  return payload;
};

export const WebSocketMiddleware = store => {
  return next => action => {
    let socketHandle = store.getState().websocket.socketHandle;
    let accessToken = store.getState().game.accessToken;

    switch (action.type) {
      case ActionTypes.USER_LOGIN_ACTION:
        socketHandle.send(JSON.stringify(action.payload));
        break;
      case ActionTypes.JOIN_ROOM_ACTION:
        socketHandle.send(JSON.stringify(authenticateMessage(action.payload, accessToken)));
        break;
      case ActionTypes.CHAT_MESSAGE_ACTION:
        socketHandle.send(JSON.stringify(authenticateMessage(action.payload, accessToken)));
        break;
      case ActionTypes.SUBMIT_CAPTION_ACTION:
        socketHandle.send(JSON.stringify(authenticateMessage(action.payload, accessToken)));
        break;
      default:
        break;
    }

    return next(action);
  };
};

export const WebSocketInit = store => {
  const ws = new Sockette(WEBSOCKET_HOST, {
    timeout: 5000,
    maxAttempts: 3,
    onopen: e => console.log("Connected!", e),
    onmessage: e => {
      console.log("Received:", e);
      let jsonResponse = JSON.parse(e.data);

      switch (jsonResponse.type) {
        case Responses.USER_LOGIN:
          store.dispatch(UserLoginResponseAction(jsonResponse.access_token, jsonResponse.user));
          break;
        case Responses.JOIN_ROOM:
          store.dispatch(JoinRoomResponseAction(jsonResponse.access_token, jsonResponse.room));
          store.dispatch(push("/game"));
          break;
        case Responses.CHAT_MESSAGE:
          store.dispatch(ChatMessageResponseAction(
            jsonResponse.user_id, jsonResponse.username, jsonResponse.message_text));
          break;
        case Responses.GAME_STARTED:
          store.dispatch(GameStartedResponseAction(jsonResponse.game));
          break;
        case Responses.ROUND_STARTED:
          store.dispatch(RoundStartedResponseAction(jsonResponse.round));
          break;
        case Responses.SUBMIT_CAPTION:
          store.dispatch(CaptionSubmittedResponseAction(jsonResponse.saved, jsonResponse.errors))
          break;
        case Responses.SUBMISSION_CLOSED:
          store.dispatch(SubmissionClosedResponseAction(jsonResponse.captions));
          break;
        default:
          console.log("Not covered: " + jsonResponse);
          break;
      }
    },
    onreconnect: e => console.log("Reconnecting...", e),
    onmaximum: e => console.log("Stop Attempting!", e),
    onclose: e => {
      if(e.code === ECONNREFUSED) {
        store.dispatch(push("/error/503"));
      }
    },
    onerror: e => console.log("Error: " + e)
  });

  store.dispatch(WebSocketInitializedAction(ws));
};
