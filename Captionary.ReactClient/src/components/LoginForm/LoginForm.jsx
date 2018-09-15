import React from "react";
import { Button } from "reactstrap";
import { connect } from "react-redux";

import { UserLoginAction } from "../../actions/WebSocketActions";

import Input from "../Input/Input";
import "./LoginForm.css";

const mapDispatchToProps = dispatch => {
  return {
    UserLoginAction: username => dispatch(UserLoginAction(username))
  };
};

class LoginFormComponent extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      username: ""
    };

    this.handleNameChange = this.handleNameChange.bind(this);
    this.onSubmit = this.onSubmit.bind(this);
  }

  handleNameChange(e) {
    this.setState({ username: e.target.value });
  }

  onSubmit(e) {
    e.preventDefault();
    this.props.UserLoginAction(this.state.username);
  }

  render() {
    return (
      <div className="login-form">
        <Input
          onChange={this.handleNameChange}
          value={this.state.playerName}
          placeholder="What is your name?"
        />

        <Button color="success" block onClick={this.onSubmit}>
          Play
        </Button>
      </div>
    );
  }
}

const LoginForm = connect(
  null,
  mapDispatchToProps
)(LoginFormComponent);

export default LoginForm;
