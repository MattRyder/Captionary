import React from 'react';
import { withRouter } from 'react-router-dom';
import LoginForm from '../../LoginForm/LoginForm';

import "./HomeView.css";

class HomeView extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            inputPlayerName: ""
        };

    }

    render() {
        return (
            <div className="home-view">
                <LoginForm inputPlayerName={this.state.inputPlayerName} />
            </div>
        );
    }
};

export default withRouter(HomeView);