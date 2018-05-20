import React from 'react';
import { Switch, Route } from 'react-router-dom';

import GameView from '../Views/GameView/GameView';
import HomeView from '../Views/HomeView/HomeView';

export default class Content extends React.Component {
    constructor(props) {
        super(props);

        this.state = { };
    }

    render() {
        return (
            <Switch>
                <Route exact path="/" component={HomeView} />
                <Route exact path="/game" component={GameView} />
            </Switch>
        );
    }
}