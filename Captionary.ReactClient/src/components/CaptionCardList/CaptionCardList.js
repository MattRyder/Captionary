import React from 'react'
import PropTypes from 'prop-types'
import CaptionCard from "../CaptionCard/CaptionCard.js"
import './CaptionCardList.css'

/**
 * A component that contains a collection of CaptionCard components,
 * and allows for the user selection of one of the cards. 
 */
class CaptionCardList extends React.Component {

    constructor(props) {
      super(props)
    
      this.state = {
         cards: props.cards || [],
         selectedCardIndex: undefined,
      }

      this.handleClick = this.handleClick.bind(this);
    }

    handleClick(_selectedCardIndex, e) {
        this.setState({ selectedCardIndex: _selectedCardIndex });
    }

    render() {
        var t = this;
        return (
            <div className="caption-card-list" >        
                {this.state.cards.map(function(cardText, i) {
                    return <CaptionCard key={i} text={cardText} handleClick={t.handleClick.bind(this, i)}/>
                })}
            </div>
        )
    }
}

CaptionCardList.propTypes = {
    cards: PropTypes.arrayOf(PropTypes.instanceOf(CaptionCard)),
    selectedCardIndex: PropTypes.number
}

export default CaptionCardList;