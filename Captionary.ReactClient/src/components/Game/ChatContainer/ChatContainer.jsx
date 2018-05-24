import React from 'react';
import PropTypes from 'prop-types';
import { Button } from 'reactstrap';
import { ChatFeed, Message } from 'react-chat-ui';
import Input from '../../Input/Input';

import './ChatContainer.css';

const KEY_ENTER = 13;

const SERVER_ACTION_SEND_MESSAGE = "SendMessage";
const SERVER_ACTION_RECEIVE_MESSAGE = "ReceiveMessage";

/**
 * A component that renders an area for users to chat
 */
export default class ChatContainer extends React.Component {

    constructor(props) {
        super(props)

        this.state = {
            messageIdIdx: 0,
            messageText: "",
            messages: []
        }

        this.handleSubmitClick = this.handleSubmitClick.bind(this);
        this.handleInputChange = this.handleInputChange.bind(this);
        this.handleInputKeyPress = this.handleInputKeyPress.bind(this);
        this.sendMessage = this.sendMessage.bind(this);
        this.receiveMessage = this.receiveMessage.bind(this);
    }

    componentWillMount() {
        this.props.hubConnection.on(SERVER_ACTION_RECEIVE_MESSAGE, (message) => {
            var msg = this.createMessage(message.senderId, message.senderName, message.message);
            this.receiveMessage(msg);
        });
    }

    handleInputChange(e) {
        this.setState({ messageText: e.target.value });
    }

    handleInputKeyPress(e) {
        if (e.charCode === KEY_ENTER) {
            this.sendMessage();
        }
    }

    handleSubmitClick(e) {
        this.sendMessage();
    }

    sendMessage() {
        if (this.state.messageText.length <= 0) {
            return;
        }

        var msg = this.createMessage(0, this.props.name, this.state.messageText);

        this.props.hubConnection
            .invoke(SERVER_ACTION_SEND_MESSAGE, msg)
            .catch(err => { console.error("Failed to send message: " + err) });

        this.setState({ messageText: "" });
    }

    receiveMessage(message) {
        var msg = new Message(message);

        this.setState((prevState, props) => {
            return {
                messages: prevState.messages.concat(msg)
            }
        });
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
            <div className='chat-container'>
                <ChatFeed
                    showSenderName
                    messages={this.state.messages}
                    hasInputField={false}
                />

                <div className="chat-input">
                    <Input placeholder="Say..."
                        onChange={this.handleInputChange}
                        onKeyPress={this.handleInputKeyPress}
                        value={this.state.messageText} />

                    <span className="input-group-btn">
                        <Button color="primary" onClick={this.handleSubmitClick}>send</Button>
                    </span>
                </div>
            </div>
        )
    }
}

ChatContainer.propTypes = {
    messageText: PropTypes.string,
    messages: PropTypes.arrayOf(Message),
    messageIdIdx: PropTypes.number
}