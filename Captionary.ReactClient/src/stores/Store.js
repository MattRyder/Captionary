import { createStore, combineReducers, applyMiddleware } from 'redux';
import { routerReducer, routerMiddleware } from 'react-router-redux';
import createHistory from 'history/createBrowserHistory';

import { WebSocketMiddleware, WebSocketInit } from '../middlewares/WebSocketMiddleware';
import GameReducer from '../reducers/GameReducer';
import WebSocketReducer from '../reducers/WebSocketReducer';

const history = createHistory();
const routingMiddleware = routerMiddleware(history);

const store = createStore(
    combineReducers({
        game: GameReducer,
        routing: routerReducer,
        websocket: WebSocketReducer
    }),
    applyMiddleware(routingMiddleware, WebSocketMiddleware)
);

WebSocketInit(store);

export const gameStore = store;
export const gameHistory = history; 