import React from 'react';
import { Button } from 'reactstrap';
import connect from 'react-redux/lib/connect/connect';
import Input from '../../Input/Input';
import { SubmitCaptionAction } from '../../../actions/SubmitCaptionAction';

import './CaptionInputForm.css';

const mapStateToProps = (state) => {
    return {
        sessionInfo: state.game.sessionInfo,
        roundId: state.round.roundId
    };
};

const mapDispatchToProps = (dispatch) => {
    return {
        SubmitCaptionAction: (roomId, roundId, captionText) =>
            dispatch(SubmitCaptionAction(roomId, roomId, captionText))
    }
}

/**
 * A component that records/resets the caption
 */
class _CaptionInputForm extends React.Component {

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
        this.setState({ captionText: e.target.value });
    }

    handleSubmit(e) {
        this.props.SubmitCaptionAction(
            this.props.sessionInfo.roomId, this.props.roundId, this.state.captionText);
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
                <Button color="primary" block onClick={this.handleSubmit}>
                    Submit
            </Button>
                <Button color="secondary" block onClick={this.clearState}>
                    Reset Input
            </Button>
            </div>

        )
    }
};

const CaptionInputForm = connect(mapStateToProps, mapDispatchToProps)(_CaptionInputForm);
export default CaptionInputForm;