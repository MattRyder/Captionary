import { ROUND_STARTED_ACTION } from '../constants/ActionTypes';

export const RoundStartedAction = (roundId, roundNumber, imageUrl) => ({
    type: ROUND_STARTED_ACTION,
    payload: {
        roundId: roundId,
        roundNumber: roundNumber,
        imageUrl: imageUrl
    }
});