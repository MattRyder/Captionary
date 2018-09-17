import React from "react";
import { connect } from "react-redux";

import ImageContainer from "../../Game/ImageContainer/ImageContainer";
import CaptionInputForm from "../../Game/CaptionInputForm/CaptionInputForm";
import CaptionCardList from "../../Game/CaptionCardList/CaptionCardList";
import ChatContainer from "../../Game/ChatContainer/ChatContainer";

import "./GameView.css";

const mapStateToProps = state => {
  return { room: state.game.room };
};

class GameViewComponent extends React.Component {
  render() {
    return (
      <div className="game-container">
        <div className="game">
          {/* <CaptionCardList />,
                    <ImageContainer imageCentered={true} />,
                    <CaptionInputForm /> */}
        </div>
        <div className="game-chat">
          <ChatContainer />
        </div>
      </div>
    );
  }
}

const GameView = connect(
  mapStateToProps,
  null
)(GameViewComponent);
export default GameView;
