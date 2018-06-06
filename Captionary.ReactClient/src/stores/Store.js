import { createStore, combineReducers, applyMiddleware } from 'redux';
import { routerReducer, routerMiddleware } from 'react-router-redux';
import createHistory from 'history/createBrowserHistory';

import GameReducer from '../reducers/GameReducer';
import RoundReducer from '../reducers/RoundReducer';
import { SignalrMiddleware, SignalrInit } from '../middlewares/SignalrMiddleware';

const history = createHistory();
const routingMiddleware = routerMiddleware(history);

const store = createStore(
    combineReducers({
        game: GameReducer,
        routing: routerReducer,
        round: RoundReducer
    }),
    applyMiddleware(SignalrMiddleware, routingMiddleware)
);

SignalrInit(store);

export const gameStore = store;
export const gameHistory = history; 