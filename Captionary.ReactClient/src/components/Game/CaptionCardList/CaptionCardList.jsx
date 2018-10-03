import React from "react";
import { connect } from 'react-redux';

import { ListGroup, ListGroupItem } from "reactstrap";
// import PropTypes from "prop-types";
import { SubmitVoteAction } from '../../../actions/WebSocketActions';
import "./CaptionCardList.css";

const mapStateToProps = (state) => {
  return {
    votedForCaptionId: state.game.votedForCaptionId
  }
}

const mapDispatchToProps = (dispatch) => {
  return {
    SubmitVoteAction: (captionId) => dispatch(SubmitVoteAction(captionId))
  }
};

/**
 * A component that contains a collection of CaptionCard components,
 * and allows for the user selection of one of the cards.
 */
class CaptionCardListComponent extends React.Component {
  
  handleClick(captionId, e) {
    if(!this.props.votedForCaptionId) {
      this.props.SubmitVoteAction(captionId);
    }
  }

  render() {
    return this.props.captions ? (
      <div>
        <h3>Captions</h3>
        <ListGroup>
          {this.props.captions ? this.props.captions.map((caption) => {
            return (
              <ListGroupItem key={caption.id} tag="a" href="#" action 
                             color={this.props.votedForCaptionId === caption.id ? "success" : ""}
                             onClick={this.handleClick.bind(this, caption.id)}>
                {caption.text}
              </ListGroupItem>
            );
          }) : ""}
        </ListGroup>
      </div>
    ) : null;
  }
}

// CaptionCardList.propTypes = {
//     cards: PropTypes.arrayOf(PropTypes.instanceOf(CaptionCard)),
//     selectedCardIndex: PropTypes.number
// }

const CaptionCardList = connect(mapStateToProps, mapDispatchToProps)(CaptionCardListComponent);
export default CaptionCardList;
