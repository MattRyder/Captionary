import React from "react";
import { ListGroup, ListGroupItem } from "reactstrap";
// import PropTypes from "prop-types";
import "./CaptionCardList.css";

/**
 * A component that contains a collection of CaptionCard components,
 * and allows for the user selection of one of the cards.
 */
class CaptionCardList extends React.Component {
  constructor(props) {
    super(props);
  }

  handleClick(e, captionId) {
      debugger;
  }

  render() {
    return (
      <div>
        <h3>Captions</h3>
        <ListGroup>
          {this.props.captions ? this.props.captions.map((caption) => {
            return (
              <ListGroupItem key={caption.id} tag="a" href="#" action 
                             onClick={this.handleClick.bind(this, caption.id)}>
                {caption.text}
              </ListGroupItem>
            );
          }) : ""}
        </ListGroup>
      </div>
    );
  }
}

// CaptionCardList.propTypes = {
//     cards: PropTypes.arrayOf(PropTypes.instanceOf(CaptionCard)),
//     selectedCardIndex: PropTypes.number
// }

export default CaptionCardList;
