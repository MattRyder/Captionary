import React from 'react';
import PropTypes from 'prop-types';
import { Button } from 'reactstrap';
import connect from 'react-redux/lib/connect/connect';
import Input from '../../Input/Input';
import { SubmitCaptionAction } from '../../../actions/WebSocketActions';

import './CaptionInputForm.css';

const mapDispatchToProps = (dispatch) => {
    return {
        SubmitCaptionAction: (captionText) => dispatch(SubmitCaptionAction(captionText))
    }
}

/**
 * A component that records/resets the caption
 */
class CaptionInputFormComponent extends React.Component {

    constructor(props) {
        super(props)

        this.state = this.baseState = {
            captionText: ""
        };

        this.clearState = this.clearState.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleCaptionTextChanged = this.handleCaptionTextChanged.bind(this);
    }

    handleCaptionTextChanged(e) {
        if(this.props.canSubmitCaption) {
            this.setState({ captionText: e.target.value });
        }
    }

    handleSubmit(e) {
        this.props.SubmitCaptionAction(this.state.captionText);
    }

    clearState() {
        this.setState(this.baseState);
    }

    render() {
        return (
            <div className="caption-input">
                <div className="form-group">
                    <Input placeholder="Enter caption"
                        onChange={this.handleCaptionTextChanged}
                        value={this.state.captionText} />
                </div>
                <Button color="primary" block 
                        disabled={!this.props.canSubmitCaption} 
                        onClick={this.handleSubmit}>
                    Submit
            </Button>
                <Button color="secondary" block 
                        disabled={!this.props.canSubmitCaption}
                        onClick={this.clearState}>
                    Reset Input
            </Button>
            </div>

        )
    }
};

CaptionInputFormComponent.PropTypes = {
    canSubmitCaption: PropTypes.bool
};

const CaptionInputForm = connect(null, mapDispatchToProps)(CaptionInputFormComponent);
export default CaptionInputForm;