import * as ActionTypes from "../constants/ActionTypes";

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
      return Object.assign({}, state, { room: action.payload.room });
    // case ActionTypes.RECEIVE_CHAT_MESSAGE_ACTION: {
    //   return {
    //     ...state,
    //     chatMessages: [...state.chatMessages, action.payload.message]
    //   };
    // }
    default:
      return state;
  }
};

export default GameReducer;
