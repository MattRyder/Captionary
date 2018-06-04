import React from 'react';
import { withRouter } from 'react-router-dom';
import LoginForm from '../../LoginForm/LoginForm';

import "./HomeView.css";

class HomeView extends React.Component {
    render() {
        return (
            <div className="home-view">
                <LoginForm roomId={this.props.match.params.id} />
            </div>
        );
    }
};

export default withRouter(HomeView);