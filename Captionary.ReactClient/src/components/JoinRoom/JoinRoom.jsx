import React from 'react';
import PropTypes from 'prop-types';
import { connect } from "react-redux";
import { Button } from 'reactstrap';

import { JoinRoomAction } from '../../actions/WebSocketActions';

import './JoinRoom.css';

const mapDispatchToProps = dispatch => {
    return {
        JoinRoomAction: roomId => dispatch(JoinRoomAction(roomId))
    }
}

class JoinRoomComponent extends React.Component {

    constructor(props) {
        super(props);
        
        this.state = {
            roomId: this.props.roomId
        };

        this.onClick = this.onClick.bind(this);
    }

    onClick() {
        this.props.JoinRoomAction(this.state.roomId);
    }

    render() {
        return (
            <div className="join-room">
                <div className="room-info">
                    <h2>Room</h2>
                    <p>{this.state.roomId ? this.state.roomId : "No room requested."}</p>
                </div>
                <Button color="success" block onClick={this.onClick}>
                Join
                </Button>
            </div>
        )
    }
};

const JoinRoom = connect(null, mapDispatchToProps)(JoinRoomComponent);

JoinRoom.propTypes = {
    roomId: PropTypes.string
};

export default JoinRoom;