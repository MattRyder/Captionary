import React from "react";
import { connect } from "react-redux";
import PropTypes from "prop-types";
import { Button } from "reactstrap";
import { ChatFeed, Message } from "react-chat-ui";

import Input from "../../Input/Input";
import { SendChatMessageAction } from "../../../actions/SendChatMessageAction";

import "./ChatContainer.css";

const KEY_ENTER = 13;

const mapStateToProps = state => {
  return {
    sessionInfo: state.game.sessionInfo,
    chatMessages: state.game.chatMessages
  };
};

const mapDispatchToProps = dispatch => {
  return {
    SendChatMessageAction: (roomId, message) =>
      dispatch(SendChatMessageAction(roomId, message))
  };
};

/**
 * A component that renders an area for users to chat
 */
class ChatContainerComponent extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      messageText: ""
    };

    this.handleInputChange = this.handleInputChange.bind(this);
    this.handleInputKeyPress = this.handleInputKeyPress.bind(this);
    this.sendMessage = this.sendMessage.bind(this);
  }

  handleInputChange(e) {
    this.setState({ messageText: e.target.value });
  }

  handleInputKeyPress(e) {
    if (e.charCode === KEY_ENTER) {
      this.sendMessage();
    }
  }

  sendMessage() {
    if (this.state.messageText.length <= 0) {
      return;
    }

    var msg = this.createMessage(0, this.props.sessionInfo.playerName, this.state.messageText);

    this.props.SendChatMessageAction(this.props.sessionInfo.roomId, msg);

    this.setState({ messageText: "" });
  }

  createMessage(id, senderName, message) {
    return new Message({
      id: id,
      senderName: senderName,
      message: message
    });
  }

  render() {
    return (
      <div className="chat-container">
        <ChatFeed
          showSenderName
          messages={this.props.chatMessages}
          hasInputField={false}
        />

        <div className="chat-input">
          <Input
            placeholder="Say..."
            onChange={this.handleInputChange}
            onKeyPress={this.handleInputKeyPress}
            value={this.state.messageText}
          />

          <span className="input-group-btn">
            <Button color="primary" onClick={this.sendMessage}>
              send
            </Button>
          </span>
        </div>
      </div>
    );
  }
}

ChatContainerComponent.propTypes = {
  messageText: PropTypes.string,
  messages: PropTypes.arrayOf(Message),
  messageIdIdx: PropTypes.number
};

const ChatContainer = connect(
  mapStateToProps,
  mapDispatchToProps
)(ChatContainerComponent);
export default ChatContainer;
