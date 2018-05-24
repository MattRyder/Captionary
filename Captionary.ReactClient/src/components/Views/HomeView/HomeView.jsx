import React from 'react';
import { withRouter } from 'react-router-dom';
import LoginForm from '../../LoginForm/LoginForm';

import "./HomeView.css";

const SERVER_ACTION_LOGIN = "PlayerLogin";

class HomeView extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            inputPlayerName: ""
        };

        this.props.hubConnection.on('JoinGame', (gameId) => {
            this.props.history.push("/game/"+gameId);
        });

        this.handleLogin = this.handleLogin.bind(this);
    }

    handleLogin(playerName) {
        this.props.hubConnection
            .invoke(SERVER_ACTION_LOGIN, playerName, 12240)
            .catch(err => { console.error("Failed to send message: " + err) });

    }

    render() {
        return (
            <div className="home-view">
                <LoginForm
                    inputPlayerName={this.state.inputPlayerName}
                    onSubmit={this.handleLogin} />
            </div>
        );
    }
};

export default withRouter(HomeView);