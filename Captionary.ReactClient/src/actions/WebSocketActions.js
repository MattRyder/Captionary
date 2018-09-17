import {
  WEBSOCKET_INITIALIZED_ACTION,
  USER_LOGIN_ACTION,
  JOIN_ROOM_ACTION,
  CHAT_MESSAGE_ACTION,
} from "../constants/ActionTypes";

import {
  USER_LOGIN_RESPONSE_ACTION,
  JOIN_ROOM_RESPONSE_ACTION,
  CHAT_MESSAGE_RESPONSE_ACTION
} from "../constants/ActionTypes";

export const WebSocketInitializedAction = socketHandle => ({
  type: WEBSOCKET_INITIALIZED_ACTION,
  payload: {
    socketHandle: socketHandle
  }
});

export const UserLoginAction = username => ({
  type: USER_LOGIN_ACTION,
  payload: {
    UserLogin: {
      username: username
    }
  }
});

export const UserLoginResponseAction = user => ({
  type: USER_LOGIN_RESPONSE_ACTION,
  payload: { user: user }
});

export const JoinRoomAction = roomId => ({
  type: JOIN_ROOM_ACTION,
  payload: {
    JoinRoom: {
      room_id: roomId
    }
  }
});

export const JoinRoomResponseAction = room => ({
  type: JOIN_ROOM_RESPONSE_ACTION,
  payload: { room: room }
});

export const ChatMessageAction = messageText => ({
  type: CHAT_MESSAGE_ACTION,
  payload: {
    ChatSent: { message_text: messageText }
  }
});

export const ChatMessageResponseAction = (userId, username, messageText) => ({
  type: CHAT_MESSAGE_RESPONSE_ACTION,
  payload: {
    userId: userId,
    username: username,
    messageText: messageText
  }
});