import { SEND_CHAT_MESSAGE_ACTION } from '../constants/ActionTypes';

export const SendChatMessageAction = (message) => ({
    type: SEND_CHAT_MESSAGE_ACTION,
    payload: { message: message }
});