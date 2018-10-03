import {
  WEBSOCKET_INITIALIZED_ACTION,
  USER_LOGIN_ACTION,
  JOIN_ROOM_ACTION,
  CHAT_MESSAGE_ACTION,
  SUBMIT_CAPTION_ACTION,
  ROUND_STARTED_RESPONSE_ACTION,
  SUBMISSION_CLOSED_RESPONSE_ACTION,
  SUBMIT_VOTE_ACTION,
} from "../constants/ActionTypes";

import {
  USER_LOGIN_RESPONSE_ACTION,
  JOIN_ROOM_RESPONSE_ACTION,
  CHAT_MESSAGE_RESPONSE_ACTION,
  GAME_STARTED_RESPONSE_ACTION,
  CAPTION_SUBMITTED_RESPONSE_ACTION,
  VOTE_SUBMITTED_RESPONSE_ACTION
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

export const UserLoginResponseAction = (accessToken, user) => ({
  type: USER_LOGIN_RESPONSE_ACTION,
  payload: { accessToken: accessToken, user: user }
});

export const JoinRoomAction = roomId => ({
  type: JOIN_ROOM_ACTION,
  payload: {
    JoinRoom: {
      room_id: roomId
    }
  }
});

export const JoinRoomResponseAction = (updatedAccessToken, room) => ({
  type: JOIN_ROOM_RESPONSE_ACTION,
  payload: { accessToken: updatedAccessToken, room: room }
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

export const GameStartedResponseAction = (game) => ({
  type: GAME_STARTED_RESPONSE_ACTION,
  payload: { game: game }
});

export const RoundStartedResponseAction = (round) => ({
  type: ROUND_STARTED_RESPONSE_ACTION,
  payload: { round: round }
});

export const SubmitCaptionAction = (captionText) => ({
  type: SUBMIT_CAPTION_ACTION,
  payload: {
    SubmitCaption: { caption_text: captionText }
  }
});

export const CaptionSubmittedResponseAction = (saved, errors) => ({
  type: CAPTION_SUBMITTED_RESPONSE_ACTION,
  payload: { saved: saved, errors: errors }
});

export const SubmissionClosedResponseAction = (captions) => ({
  type: SUBMISSION_CLOSED_RESPONSE_ACTION,
  payload: { captions: captions }
});

export const SubmitVoteAction = (captionId) => ({
  type: SUBMIT_VOTE_ACTION,
  payload: { 
    CaptionVote: { caption_id: captionId }
  }
});

export const VoteSubmittedResponseAction = (captionId) => ({
  type: VOTE_SUBMITTED_RESPONSE_ACTION,
  payload: { captionId: captionId }
})