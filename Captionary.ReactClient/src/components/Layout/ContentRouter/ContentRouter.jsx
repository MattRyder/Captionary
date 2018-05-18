import React from 'react';
import { Switch, Route } from 'react-router-dom';

import Game from '../../Views/Game/Game';

export default class Content extends React.Component {
    constructor(props) {
        super(props);

        this.state = { };
    }

    render() {
        return (
            <Switch>
                <Route exact path="/" component={Game} />
            </Switch>
        );
    }
}