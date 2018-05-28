import React from 'react';
import ReactDOM from 'react-dom';
import { ConnectedRouter } from 'react-router-redux';
import { Provider } from 'react-redux';
import { gameStore, gameHistory } from './stores/Store';

import 'bootstrap/dist/css/bootstrap.css';
import './index.css';

import App from './App';

ReactDOM.render(
    <Provider store={ gameStore }>
        <ConnectedRouter history={ gameHistory }>
            <App />
        </ConnectedRouter>
    </Provider>,
    document.getElementById('captionary'));
