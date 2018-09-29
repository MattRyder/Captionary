import React from "react";
import { connect } from "react-redux";
import { push } from "react-router-redux";

import ImageContainer from "../../Game/ImageContainer/ImageContainer";
import CaptionInputForm from "../../Game/CaptionInputForm/CaptionInputForm";
import CaptionCardList from "../../Game/CaptionCardList/CaptionCardList";
import ChatContainer from "../../Game/ChatContainer/ChatContainer";

import "./GameView.css";

const mapStateToProps = state => {
  return { 
    room: state.game.room,
    game: state.game.game,
    round: state.game.round,
    hasSubmittedCaption: state.game.hasSubmittedCaption
  };
};

const mapDispatchToProps = dispatch => {
  return {
    redirectToHome: () => dispatch(push("/"))
  }
};

class GameViewComponent extends React.Component {

  componentDidMount() {
    if(!this.props.room) {
      this.props.redirectToHome();
    }
  }

  render() {
    return (
      <div className="game-container">
        <div className="game">
          {/* <CaptionCardList />, */}

          <ImageContainer
            imageUrl={this.props.round ? this.props.round.image_url : null}
            imageCentered={true} />

          <CaptionInputForm
            canSubmitCaption={!this.props.hasSubmittedCaption} />
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
