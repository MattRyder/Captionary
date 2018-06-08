import { SEND_CHAT_MESSAGE_ACTION } from "../constants/ActionTypes";

export const SendChatMessageAction = (roomId, message) => ({
  type: SEND_CHAT_MESSAGE_ACTION,
  payload: { roomId: roomId, message: message }
});
