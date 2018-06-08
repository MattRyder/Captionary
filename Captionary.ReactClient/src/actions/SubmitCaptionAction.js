import { SUBMIT_CAPTION_ACTION } from '../constants/ActionTypes';

export const SubmitCaptionAction = (roomId, roundId, captionText) => ({
    type: SUBMIT_CAPTION_ACTION,
    payload: {
        roomId: roomId,
        roundId: roundId,
        captionText: captionText
    }
});