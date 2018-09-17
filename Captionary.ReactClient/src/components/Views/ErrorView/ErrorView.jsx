import React from "react";
import Anime from "react-anime";

import "./ErrorView.css";

import ErrorIcon from "./error-icon.svg";

const E_NOT_FOUND = 404;
const E_INTERNAL_SERVER_ERROR = 500;
const E_SERVICE_UNAVAILABLE = 503;

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
      });
    }
  }

  getErrorMessage() {
    switch (this.props.match.params.id) {
      case E_NOT_FOUND: {
        return {
          title: "Page Not Found",
          message: "The page you requested doesn't exist."
        };
      }
      case E_INTERNAL_SERVER_ERROR: {
        return {
          title: "Internal Server Error",
          message: "Something is broken inside the server. Apologies."
        };
      }
      case E_SERVICE_UNAVAILABLE: {
        return {
          title: "Failed to connect to the Captionary Server",
          message: "The server is nowhere to be seen."
        };
      }
      default:
        break;
    }
  }

  render() {
    return (
      <div className="error-container">
        <a href="/" className="error-image">
          <Anime
            rotate={360}
            loop={true}
            duration={8000}
            easing="easeInOutCubic"
            direction="alternate"
          >
            <img alt="" className="img-fluid" src={ErrorIcon} />
          </Anime>
        </a>
        <h1>{this.state.title}</h1>
        <p>{this.state.message}</p>
      </div>
    );
  }
}
