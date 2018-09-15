import React from 'react';
import { connect } from 'react-redux';

import { withRouter } from 'react-router-dom';
import LoginForm from '../../LoginForm/LoginForm';
import JoinRoom from '../../JoinRoom/JoinRoom';

import "./HomeView.css";

const mapStateToProps = state => {
    return { user: state.game.user };
};

class HomeViewComponent extends React.Component {
    
    constructor(props) {
        super(props);

        this.state = {
            requestedRoomId: this.props.match.params.id
        };
    }

    render() {
        return (
            <div className="home-view">
                { this.props.user ? (<JoinRoom roomId={this.state.requestedRoomId} />) : "" }
                { !this.props.user ? <LoginForm /> : "" }
            </div>
        );
    }
};

const HomeView = connect(mapStateToProps, null)(HomeViewComponent);
export default withRouter(HomeView);