import React from 'react';

import ImageContainer from "../../Game/ImageContainer/ImageContainer";
import CaptionInput from "../../Game/CaptionInput/CaptionInput"
import CaptionCardList from "../../Game/CaptionCardList/CaptionCardList";
import ChatContainer from "../../Game/ChatContainer/ChatContainer";

import './Game.css'

export default class Game extends React.Component {
    constructor(props) {
        super(props);

        this.state = {

        }
    }

    render() {
        return (
            <div className="game-container">
                <div className="game">
                    <CaptionCardList/>
                    <ImageContainer imageCentered={true} />
                    <CaptionInput />
                </div>
                <div className="game-chat">
                    <ChatContainer name={"Matt"} />
                </div>
            </div>
        );
    }
}