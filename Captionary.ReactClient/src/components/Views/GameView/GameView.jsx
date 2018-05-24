import React from 'react';

import ImageContainer from "../../Game/ImageContainer/ImageContainer";
import CaptionInputForm from "../../Game/CaptionInputForm/CaptionInputForm";
import CaptionCardList from "../../Game/CaptionCardList/CaptionCardList";
import ChatContainer from "../../Game/ChatContainer/ChatContainer";

import './GameView.css'

const SERVER_ACTION_PLAYER_CONNECTED = "PlayerConnected";
const SERVER_ACTION_PLAYER_DISCONNECTED = "PlayerDisconnected";

export default class GameView extends React.Component {
    constructor(props) {
        super(props);
    }

    componentWillMount() {
        this.props.hubConnection.on(SERVER_ACTION_PLAYER_CONNECTED, (playerName) => {
            console.log("Player connected to Game: " +playerName);
        });

        this.props.hubConnection.on(SERVER_ACTION_PLAYER_DISCONNECTED, (playerName) => {
            console.log("Player disconnected from Game: " +playerName);
        });
    }

    render() {
        return (
            <div className="game-container">
                <div className="game">
                    <CaptionCardList />
                    <ImageContainer imageCentered={true} />
                    <CaptionInputForm />
                </div>
                <div className="game-chat">
                    <ChatContainer hubConnection={this.props.hubConnection} name={"Matt"} />
                </div>
            </div>
        );
    }
}