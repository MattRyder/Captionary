import React from 'react'
import PropTypes from 'prop-types'
import { Button } from 'reactstrap'

import './CaptionInput.css'

const CaptionInputClassState = {
    DEFAULT: "",
    FOCUSED: "is-focused",
    DISABLED: "is-disabled"
};

/**
 * A component that accepts a user's input
 */
class CaptionInput extends React.Component {

    constructor(props) {
      super(props)
    
      this.state = {
         value: "",
         class: CaptionInputClassState.DEFAULT
      };

      this.baseState = this.state;

      this.handleChange = this.handleChange.bind(this);
      this.handleFocus = this.handleFocus.bind(this);
      this.handleBlur = this.handleBlur.bind(this); 
      this.handleSubmit = this.handleSubmit.bind(this);   

      this.clearState = this.clearState.bind(this);
    }

    handleChange(e) {
        this.setState({ value: e.target.value });
    }

    handleFocus() {
        this.setState({ class: CaptionInputClassState.FOCUSED });
    }

    handleBlur() {
        this.setState({ class: CaptionInputClassState.DEFAULT });      
    }

    handleSubmit(e) {
        if(typeof this.props.handleSubmit === 'function') {
            this.props.handleSubmit();
        }

        this.setState({ class: CaptionInputClassState.DISABLED });
    }

    clearState() {
        this.setState(this.baseState);
    }

    render() {
      return (
        <div className={"caption-input " + this.state.class}>
            <div className="form-group">
                <input type="text" className="form-control"  placeholder="Enter caption"
                        value={this.state.value}
                        disabled={this.state.class === CaptionInputClassState.DISABLED}
                        onChange={this.handleChange} onFocus={this.handleFocus} onBlur={this.handleBlur}/>
            </div>
            <Button color="primary" block onClick={this.handleSubmit}>
                Submit
            </Button>
            <Button color="secondary" block onClick={this.clearState}>
                Reset Input
            </Button>
        </div>

      )
    }   
}

CaptionInput.propTypes = {
    handleSubmit: PropTypes.func
}

export default CaptionInput;