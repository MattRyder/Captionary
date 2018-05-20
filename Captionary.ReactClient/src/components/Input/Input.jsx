import React from 'react'

import './Input.css'

const InputState = {
    DEFAULT: "",
    FOCUSED: "is-focused",
    DISABLED: "is-disabled"
};

export default class Input extends React.Component {
    constructor(props) {
        super(props);

        this.state = this.baseState = {
            class: InputState.DEFAULT
        };

        this.handleFocus = this.handleFocus.bind(this);
        this.handleBlur = this.handleBlur.bind(this); 
        this.clearInput = this.clearInput.bind(this);
    }

    clearInput() {
        this.setState(this.baseState);
    }

    handleFocus() {
        this.setState({ class: InputState.FOCUSED });
    }

    handleBlur() {
        this.setState({ class: InputState.DEFAULT });      
    }

    render() {
        return (
            <div className="input-container">
                <input type="text" className="input"
                    value={this.props.value}
                    disabled={this.state.class === InputState.DISABLED}
                    placeholder={this.props.placeholder}
                    onChange={this.props.onChange}
                    onKeyPress={this.props.onKeyPress}
                    onFocus={this.handleFocus}
                    onBlur={this.handleBlur}/>
                <span className="underline"></span>
            </div>
        )
    }
}