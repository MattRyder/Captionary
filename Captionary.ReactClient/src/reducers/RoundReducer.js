import * as ActionTypes from '../constants/ActionTypes';

const InitialState = {
    roundId: 0,
    roundNumber: 0,
    imageUrl: ""
};

const RoundReducer = (state = InitialState, action) => {
    switch (action.type) {
        case ActionTypes.ROUND_STARTED_ACTION:
            return Object.assign({}, state, {
                roundId: action.payload.roundId,
                roundNumber: action.payload.roundNumber,
                imageUrl: action.payload.imageUrl
            });
        default: return state;
    }
};

export default RoundReducer;