import React from "react";
import { connect } from "react-redux";
import { push } from "react-router-redux";

import ImageContainer from "../../Game/ImageContainer/ImageContainer";
import CaptionInputForm from "../../Game/CaptionInputForm/CaptionInputForm";
import CaptionCardList from "../../Game/CaptionCardList/CaptionCardList";
import ChatContainer from "../../Game/ChatContainer/ChatContainer";

import GameState from "../../../constants/GameState";

import "./GameView.css";

const mapStateToProps = state => {
  return {
    gameState: state.game.gameState,
    room: state.game.room,
    game: state.game.game,
    round: state.game.round,
    captions: state.game.captions,
    hasSubmittedCaption: state.game.hasSubmittedCaption
  };
};

const mapDispatchToProps = dispatch => {
  return {
    redirectToHome: () => dispatch(push("/"))
  }
};

class GameViewComponent extends React.Component {

  constructor(props) {
    super(props);

    this.isImageContainerVisible = this.isImageContainerVisible.bind(this);
    this.isCaptionInputVisible = this.isCaptionInputVisible.bind(this);
    this.isCardListVisible = this.isCardListVisible.bind(this);
  }

  componentDidMount() {
    if(!this.props.room) {
      this.props.redirectToHome();
    }
  }

  isImageContainerVisible() {
    return this.props.gameState === GameState.ROUND_STARTING ||
            this.props.gameState === GameState.SUBMISSION_CLOSED;
  }

  isCaptionInputVisible() {
    return this.props.gameState === GameState.ROUND_STARTING;
  }

  isCardListVisible() {
    return this.props.gameState === GameState.SUBMISSION_CLOSED ||
            this.props.gameState === GameState.ROUND_FINISHED;
  }

  render() {
    return (
      <div className="game-container">
        <div className="game">
          { this.isImageContainerVisible() ? (
              <ImageContainer
                imageUrl={this.props.round ? this.props.round.image_url : null}
                imageCentered={true} />
            ) : null }

          { this.isCardListVisible() ? (
              <CaptionCardList captions={this.props.captions} />
            ) : null }

          { this.isCaptionInputVisible() ? (
            <CaptionInputForm
              canSubmitCaption={!this.props.hasSubmittedCaption} />
            ) : null }
          }
        </div>
        <div className="game-chat">
          <ChatContainer />
        </div>
      </div>
    );
  }
}

const GameView = connect(mapStateToProps, mapDispatchToProps)(GameViewComponent);
export default GameView;
