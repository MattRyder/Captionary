import React from 'react'
import PropTypes from 'prop-types'

import "./CaptionCard.css"

const CardClassState = {
    DEFAULT: "",
    SELECTED: "is-selected",
    HOVER: "is-hovered"
}

class CaptionCard extends React.Component {

    constructor(props) {
        super(props)
        
        this.state = {
            text: this.props.text,
            class: CardClassState.DEFAULT,
        }

        this.handleMouseOver = this.handleMouseOver.bind(this);
        this.handleMouseOut = this.handleMouseOut.bind(this);
        this.handleClick = this.handleClick.bind(this);
    }

    handleClick(e) {        
        if(typeof this.props.handleClick === 'function') {
            this.props.handleClick(e);
        }

        this.setState({ class: CardClassState.SELECTED });        
    }
    
    handleMouseOver(e) {
        if(this.state.class !== CardClassState.SELECTED) {
            this.setState({ class: CardClassState.HOVER });            
        }
    }

    handleMouseOut(e) {
        if(this.state.class !== CardClassState.SELECTED) {            
            this.setState({ class: CardClassState.DEFAULT });
        }        
    }

    render() {
        return (
            <div className={'caption-card ' + this.state.class}
                onClick={this.handleClick} onMouseOver={this.handleMouseOver} onMouseOut={this.handleMouseOut}>
                <p className="text">{this.state.text}</p>
            </div>
        )
    }
}

CaptionCard.propTypes = {
    handleClick: PropTypes.func
}

export default CaptionCard;