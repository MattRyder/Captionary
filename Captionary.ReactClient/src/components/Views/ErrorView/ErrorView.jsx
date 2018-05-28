import React from 'react';
import Anime from 'react-anime';

import "./ErrorView.css";

import ErrorIcon from "./error-icon.svg";

export default class ErrorView extends React.Component {

    constructor(props) {
        super(props);

        this.state = {
            title: "Something went wrong!",
            message: "And that's all we know. So far."
        };

        this.getErrorMessage = this.getErrorMessage.bind(this);
    }

    componentWillMount() {
        const errorData = this.getErrorMessage();

        if (errorData) {
            this.setState({
                title: errorData.title,
                message: errorData.message
            })
        }
    }

    getErrorMessage() {
        switch (this.props.match.params.id) {
            case "404": {
                return {
                    title: "Page Not Found",
                    message: "The page you requested doesn't exist."
                }
            }
            case "500": {
                return {
                    title: "Internal Server Error",
                    message: "Something is broken inside the server. Apologies."
                }
            }
            case "1000": {
                return {
                    title: "Failed to connect to the Captionary Server",
                    message: "The server is nowhere to be seen."
                }
            }
            default: break;
        }
    }

    render() {
        return (
            <div className="error-container">
                <a href="/" className="error-image">
                    <Anime rotate={360}
                        loop={true}
                        duration={6000}
                        easing="easeInOutCubic"
                        direction="alternate">
                        <img alt=""
                            className="img-fluid"
                            src={ErrorIcon} />
                    </Anime>
                </a>
                <h1>{this.state.title}</h1>
                <p>{this.state.message}</p>
            </div>
        )
    }
}