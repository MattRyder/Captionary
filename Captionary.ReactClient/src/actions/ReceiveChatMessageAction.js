import { Message } from 'react-chat-ui';
import { RECEIVE_CHAT_MESSAGE_ACTION } from '../constants/ActionTypes';

export const ReceiveChatMessageAction = (messageId, senderName, messageText) => ({
    type: RECEIVE_CHAT_MESSAGE_ACTION,
    payload: {
        message: new Message({
            id: messageId,
            senderName: senderName,
            message: messageText
        })
    }
});