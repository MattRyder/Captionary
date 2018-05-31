import React from 'react';
import { Button } from 'reactstrap';
import Input from '../../Input/Input';

import './CaptionInputForm.css';

const mapDispatchToProps = (dispatch) => {
    return {
        
    }
}

/**
 * A component that records/resets the caption
 */
export default class CaptionInputForm extends React.Component {

    constructor(props) {
      super(props)
    
      this.state = this.baseState = {
         captionText: ""
      };

      this.clearState = this.clearState.bind(this);
    }

    clearState() {
        this.setState(this.baseState);
    }

    render() {
      return (
        <div className="caption-input">
            <div className="form-group">
                <Input placeholder="Enter caption"
                        value={this.state.captionText} />
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