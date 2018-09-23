import * as ActionTypes from "../constants/ActionTypes";
import { Message } from 'react-chat-ui';

const InitialState = {
  user: null,
  room: null,
  chatMessages: [],
};

const GameReducer = (state = InitialState, action) => {
  switch (action.type) {
    case ActionTypes.USER_LOGIN_RESPONSE_ACTION:
      return Object.assign({}, state, { user: action.payload.user });
    case ActionTypes.JOIN_ROOM_RESPONSE_ACTION:
      return Object.assign({}, state, { 
        user: { 
          ...state.user,
          token: action.payload.updatedAccessToken
        }, 
        room: action.payload.room });
    case ActionTypes.CHAT_MESSAGE_RESPONSE_ACTION:
      let id = action.payload.userId === state.user.id ? 0 : action.payload.userId;
      var msg = new Message({
        id: id, senderName: action.payload.username, message: action.payload.messageText
      });
      return {
        ...state,
        chatMessages: [...state.chatMessages, msg]
      };

    default:
      return state;
  }
};

export default GameReducer;
