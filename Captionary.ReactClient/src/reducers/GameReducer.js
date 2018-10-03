import * as ActionTypes from "../constants/ActionTypes";
import { Message } from 'react-chat-ui';
import GameState from "../constants/GameState";

const InitialState = {
  user: null,
  room: null,
  game: null,
  gameState: GameState.DEFAULT,
  round: null,
  captions: null,
  accessToken: null,
  chatMessages: [],
  hasSubmittedCaption: false,
  votedForCaptionId: null
};

const GameReducer = (state = InitialState, action) => {
  switch (action.type) {
    case ActionTypes.USER_LOGIN_RESPONSE_ACTION:
      return Object.assign({}, state, { 
        accessToken: action.payload.accessToken,
        user: action.payload.user
      });
    case ActionTypes.JOIN_ROOM_RESPONSE_ACTION:
      return Object.assign({}, state, { 
        accessToken: action.payload.accessToken,
        room: action.payload.room
      });
    case ActionTypes.CHAT_MESSAGE_RESPONSE_ACTION:
      let id = action.payload.userId === state.user.id ? 0 : action.payload.userId;
      var msg = new Message({
        id: id,
        senderName: action.payload.username,
        message: action.payload.messageText
      });
      return {
        ...state,
        chatMessages: [...state.chatMessages, msg]
      };
    case ActionTypes.GAME_STARTED_RESPONSE_ACTION:
      return Object.assign({}, state, { 
        game: action.payload.game,
        gameState: GameState.GAME_STARTING
      });
    case ActionTypes.ROUND_STARTED_RESPONSE_ACTION:
      return Object.assign({}, state, { 
        round: action.payload.round,
        gameState: GameState.ROUND_STARTING
      });
    case ActionTypes.CAPTION_SUBMITTED_RESPONSE_ACTION:
      return Object.assign({}, state, { hasSubmittedCaption: action.payload.saved });
    case ActionTypes.SUBMISSION_CLOSED_RESPONSE_ACTION:
      return Object.assign({}, state, {
        captions: action.payload.captions,
        gameState: GameState.SUBMISSION_CLOSED
      });
    case ActionTypes.VOTE_SUBMITTED_RESPONSE_ACTION:
      return Object.assign({}, state, { votedForCaptionId: action.payload.captionId });
    default:
      return state;
  }
};

export default GameReducer;
